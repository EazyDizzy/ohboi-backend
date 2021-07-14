use bigdecimal::ToPrimitive;
use futures::future::join_all;
use maplit::btreemap;
use scraper::Html;
use sentry::types::protocol::latest::map::BTreeMap;

use crate::common::db::repository::exchange_rate::get_exchange_rate_by_code;
use crate::common::service::currency_converter::convert_from_with_rate;
use crate::local_sentry::add_category_breadcrumb;
use crate::parse::consumer::parse_page::ParsePageMessage;
use crate::parse::crawler::crawler::Crawler;
use crate::parse::crawler::mi_shop_com::MiShopComCrawler;
use crate::parse::crawler::samsung_shop_com_ua::SamsungShopComUaCrawler;
use crate::parse::db::entity::{CategorySlug, SourceName};
use crate::parse::db::repository::product::{create_if_not_exists, update_details};
use crate::parse::db::repository::source_product::link_to_product;
use crate::parse::parsed_product::{AdditionalParsedProductInfo, InternationalParsedProduct, LocalParsedProduct};
use crate::parse::queue::postpone_page_parsing;
use crate::parse::service::html_cleaner::clean_html;
use crate::parse::service::requester::{get_data, get_data_s};
use crate::SETTINGS;

pub async fn parse_page(url: &str, source: SourceName, category: CategorySlug) -> Result<(), reqwest::Error> {
    let crawler = get_crawler(&source);
    add_parse_breadcrumb(
        "in progress",
        btreemap! {
                    "crawler" => source.to_string(),
                    "category" => category.to_string(),
                },
    );

    let response = get_data(url).await?;
    let mut products = parse_html(&response, crawler);

    dedup_products(&mut products, source);

    add_parse_breadcrumb(
        "parsed",
        btreemap! {
                    "crawler" => source.to_string(),
                    "category" => category.to_string(),
                    "length" => products.len().to_string()
                },
    );

    save_parsed_products(crawler, products, category).await;

    Ok(())
}

pub async fn parse_category(source: SourceName, category: CategorySlug) -> Result<(), reqwest::Error> {
    let crawler = get_crawler(&source);

    add_parse_breadcrumb(
        "in progress",
        btreemap! {
                    "crawler" => source.to_string(),
                    "category" => category.to_string(),
                },
    );

    let mut products: Vec<LocalParsedProduct> = vec![];
    let concurrent_pages = 5; // TODO move to the db settings of specific crawler

    for url in crawler.get_next_page_urls(category) {
        for page in (1..10000).step_by(concurrent_pages) {
            let mut page_requests = vec![];
            for page in page..page + concurrent_pages {
                let url = url.replace("{page}", (page).to_string().as_ref());

                page_requests.push(get_data_s(url));
            }

            let page_responses = join_all(page_requests).await;
            let mut all_successful = true;
            // To prevent endless requests if site is down
            let mut amount_of_fails = 0;

            let mut current_page = page;

            // TODO rewrite in stream and Mutex (parse in then and call next request)
            for response in page_responses {
                match response {
                    Ok(response_data) => {
                        let parsed = parse_html(&response_data, crawler);
                        let mut amount_of_duplicates = 0;

                        parsed.iter().for_each(|parsed_product| {
                            let will_be_duplicated = products
                                .iter()
                                .any(|p| p.external_id == parsed_product.external_id);

                            if will_be_duplicated {
                                amount_of_duplicates += 1;
                            } else {
                                products.push(parsed_product.clone());
                            }
                        });
                        all_successful = all_successful
                            && !parsed.is_empty() // Some sites return empty page
                            && amount_of_duplicates != parsed.len(); // But some return the last page (samsung)
                    }
                    Err(e) => {
                        amount_of_fails += 1;
                        sentry::capture_message(
                            format!(
                                "Request for page failed[{source}]: {error:?}",
                                source = source,
                                error = e
                            ).as_str(),
                            sentry::Level::Warning,
                        );

                        let _result = postpone_page_parsing(ParsePageMessage {
                            url: url.replace("{page}", (current_page).to_string().as_ref()),
                            source,
                            category,
                        }).await;
                    }
                }

                current_page += 1;
            }

            // if last page
            if !all_successful || amount_of_fails == concurrent_pages {
                break;
            }
        }
    }

    dedup_products(&mut products, source);

    add_parse_breadcrumb(
        "parsed",
        btreemap! {
                    "crawler" => crawler.get_source().to_string(),
                    "category" => category.to_string(),
                    "length" => products.len().to_string()
                },
    );

    save_parsed_products(crawler, products, category).await;

    Ok(())
}

async fn save_parsed_products(crawler: &dyn Crawler, products: Vec<LocalParsedProduct>, category: CategorySlug) {
    let mut savings_in_progress = vec![];
    let currency = crawler.get_currency();
    let rate = get_exchange_rate_by_code(currency).unwrap().rate.to_f64().unwrap();

    for parsed_product in products {
        savings_in_progress.push(save_parsed_product(crawler, parsed_product, category, rate));

        if savings_in_progress.len() == SETTINGS.database.product_save_concurrency {
            join_all(savings_in_progress).await;
            savings_in_progress = vec![];
        }
    }

    join_all(savings_in_progress).await;
    add_parse_breadcrumb(
        "saved",
        btreemap! {
                    "crawler" => crawler.get_source().to_string(),
                    "category" => category.to_string(),
                },
    );
}

async fn save_parsed_product(crawler: &dyn Crawler, parsed_product: LocalParsedProduct, category: CategorySlug, rate: f64) {
    let international_parsed_product = InternationalParsedProduct {
        title: parsed_product.title,
        price: convert_from_with_rate(parsed_product.price, rate),
        original_price: parsed_product.price,
        available: parsed_product.available,
        external_id: parsed_product.external_id,
    };
    let product = create_if_not_exists(&international_parsed_product, category);

    if product.description.is_none() || product.images.is_none() {
        let details = extract_additional_info(
            &international_parsed_product.external_id,
            crawler,
        ).await;

        match details {
            None => {
                sentry::capture_message(
                    format!(
                        "No additional info found [{source}] for: {id}",
                        source = crawler.get_source().to_string(),
                        id = international_parsed_product.external_id
                    ).as_str(),
                    sentry::Level::Warning,
                );
            }
            Some(details) => {
                update_details(&product, &details);
            }
        }
    }

    link_to_product(&product, &international_parsed_product, crawler.get_source());
}

fn parse_html(data: &str, crawler: &dyn Crawler) -> Vec<LocalParsedProduct> {
    let document = Html::parse_document(data);

    crawler.extract_products(&document)
}

async fn extract_additional_info(external_id: &str, crawler: &dyn Crawler) -> Option<AdditionalParsedProductInfo> {
    add_parse_breadcrumb(
        "extracting additional info",
        btreemap! {
                    "crawler" => crawler.get_source().to_string(),
                    "external_id" => external_id.to_string()
                },
    );

    let url = crawler.get_additional_info_url(&external_id);
    let data = get_data(&url).await;

    match data {
        Ok(data) => {
            let document = Html::parse_document(&data);

            crawler.extract_additional_info(&document, &external_id).await
        }
        Err(e) => {
            let message = format!(
                "Request for additional data failed! [{source}] {error:?}",
                source = crawler.get_source().to_string(),
                error = e,
            );
            sentry::capture_message(message.as_str(), sentry::Level::Warning);

            None
        }
    }
}

fn dedup_products(products: &mut Vec<LocalParsedProduct>, source: SourceName) {
    let error_margin = f64::EPSILON;

    products.dedup_by(|a, b| {
        if a.external_id == b.external_id && (a.price - b.price).abs() > error_margin {
            let message = format!(
                "Warning! Same external_id, different prices. Parser: {source}, id: {id}, price1: {price1}, price2: {price2}",
                source = source.to_string(),
                id = a.external_id,
                price1 = a.price.to_string(),
                price2 = b.price.to_string()
            );
            sentry::capture_message(message.as_str(), sentry::Level::Warning);
        }

        a.external_id == b.external_id
    });
}

fn add_parse_breadcrumb(message: &str, data: BTreeMap<&str, String>) {
    add_category_breadcrumb(message, data, "parse".into());
}

fn get_crawler(source: &SourceName) -> &dyn Crawler {
    match source {
        SourceName::MiShopCom => {
            &MiShopComCrawler {}
        }
        SourceName::SamsungShopComUa => {
            &SamsungShopComUaCrawler {}
        }
    }
}
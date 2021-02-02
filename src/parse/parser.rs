use futures::future::*;
use maplit::*;
use scraper::Html;
use sentry::types::protocol::latest::map::BTreeMap;

use crate::local_sentry::add_category_breadcrumb;
use crate::parse::consumer::parse_page::ParsePageMessage;
use crate::parse::crawler::crawler::Crawler;
use crate::parse::crawler::mi_shop_com::MiShopComCrawler;
use crate::parse::db::entity::{CategorySlug, SourceName};
use crate::parse::db::repository::product::{create_if_not_exists, update_details};
use crate::parse::db::repository::source_product::link_to_product;
use crate::parse::parsed_product::{AdditionalParsedProductInfo, ParsedProduct};
use crate::parse::queue::postpone_page_parsing;
use crate::parse::requester::get_data;
use crate::SETTINGS;

pub async fn parse_page(url: String, source: &SourceName, category: &CategorySlug) -> Result<(), reqwest::Error> {
    let crawler = get_crawler(source);
    add_parse_breadcrumb(
        "in progress",
        btreemap! {
                    "crawler" => source.to_string(),
                    "category" => category.to_string(),
                },
    );

    let response = get_data(url).await?;
    let mut products = parse_html(response, crawler.clone());

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

pub async fn parse_category(source: &SourceName, category: &CategorySlug) -> Result<(), reqwest::Error> {
    let crawler = get_crawler(source);

    add_parse_breadcrumb(
        "in progress",
        btreemap! {
                    "crawler" => source.to_string(),
                    "category" => category.to_string(),
                },
    );

    let mut products: Vec<ParsedProduct> = vec![];
    let concurrent_pages = 5; // TODO move to the db settings of specific crawler

    for url in crawler.get_next_page_urls(category) {
        for page in (1..10000).step_by(concurrent_pages) {
            let mut page_requests = vec![];
            for page in page..page + concurrent_pages {
                let url = url.replace("{page}", (page).to_string().as_ref());

                page_requests.push(get_data(url));
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
                        let parsed = parse_html(response_data, crawler.clone());

                        parsed.iter().for_each(|x| products.push(x.clone()));
                        all_successful = all_successful && !parsed.is_empty();
                    }
                    Err(e) => {
                        amount_of_fails = amount_of_fails + 1;
                        sentry::capture_message(
                            format!("Request for page failed[{}]: {:?}", source, e).as_str(),
                            sentry::Level::Warning,
                        );

                        let _result = postpone_page_parsing(ParsePageMessage {
                            url: url.replace("{page}", (current_page).to_string().as_ref()),
                            source: source.clone(),
                            category: category.clone(),
                        }).await;
                    }
                }

                current_page = current_page + 1;
            }

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

async fn save_parsed_products(crawler: &dyn Crawler, products: Vec<ParsedProduct>, category: &CategorySlug) {
    let mut savings_in_progress = vec![];

    for parsed_product in &products {
        savings_in_progress.push(save_parsed_product(crawler.clone(), parsed_product, category));

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

async fn save_parsed_product(crawler: &dyn Crawler, parsed_product: &ParsedProduct, category: &CategorySlug) {
    let product = create_if_not_exists(parsed_product, &category);

    if product.description.is_none() || product.images.is_none() {
        let details = extract_additional_info(
            parsed_product.external_id.to_string(),
            crawler,
        ).await;

        match details {
            None => {
                sentry::capture_message(
                    format!(
                        "No additional info found [{}] for: {}",
                        crawler.get_source().to_string(),
                        parsed_product.external_id.to_string()
                    ).as_str(),
                    sentry::Level::Warning,
                );
            }
            Some(details) => {
                update_details(&product, &details);
            }
        }
    }

    link_to_product(&product, parsed_product, crawler.get_source());
}

fn parse_html(data: String, crawler: &dyn Crawler) -> Vec<ParsedProduct> {
    let document = Html::parse_document(&data);

    crawler.extract_products(&document)
}

async fn extract_additional_info(external_id: String, crawler: &dyn Crawler) -> Option<AdditionalParsedProductInfo> {
    add_parse_breadcrumb(
        "extracting additional info",
        btreemap! {
                    "crawler" => crawler.get_source().to_string(),
                    "external_id" => external_id.clone()
                },
    );

    let url = crawler.get_additional_info_url(external_id.clone());
    let data = get_data(url).await;

    match data {
        Ok(data) => {
            let document = Html::parse_document(&data);

            crawler.extract_additional_info(&document, external_id).await
        }
        Err(e) => {
            let message = format!(
                "Request for additional data failed! {:?} [{}]",
                e,
                crawler.get_source().to_string()
            );
            sentry::capture_message(message.as_str(), sentry::Level::Warning);

            None
        }
    }
}

fn dedup_products(products: &mut Vec<ParsedProduct>, source: &SourceName) {
    products.dedup_by(|a, b| {
        if a.external_id == b.external_id && a.price != b.price {
            let message = format!(
                "Warning! Same external_id, different prices. Parser: {}, id: {}, price1: {}, price2: {}",
                source.to_string(),
                a.external_id,
                a.price.to_string(),
                b.price.to_string()
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
    }
}
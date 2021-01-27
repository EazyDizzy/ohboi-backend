use futures::future::*;
use inflector::Inflector;
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

pub async fn parse_page(url: String, source: &SourceName, category: &CategorySlug) -> Result<(), reqwest::Error> {
    let crawler = get_crawler(source);
    add_parse_breadcrumb(
        "in progress",
        btreemap! {
                    "crawler" => crawler.get_source().to_string(),
                    "category" => category.to_string(),
                },
    );

    let response = get_data(url).await?;
    let mut products: Vec<ParsedProduct> = vec![];
    let _parsed = parse_html(response, &mut products, crawler.clone());

    products.dedup_by(|a, b| {
        if a.external_id == b.external_id && a.price != b.price {
            let message = format!(
                "Warning! Same external_id, different prices. Parser: {}, id: {}, price1: {}, price2: {}",
                crawler.get_source().to_string().to_snake_case(),
                a.external_id,
                a.price.to_string(),
                b.price.to_string()
            );
            sentry::capture_message(message.as_str(), sentry::Level::Warning);
        }

        a.external_id == b.external_id
    });

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

pub async fn parse_category(source: &SourceName, category: &CategorySlug) -> Result<(), reqwest::Error> {
    let crawler = get_crawler(source);

    add_parse_breadcrumb(
        "in progress",
        btreemap! {
                    "crawler" => crawler.get_source().to_string(),
                    "category" => category.to_string(),
                },
    );

    let mut products: Vec<ParsedProduct> = vec![];
    let current_length = products.len();
    let concurrent_pages = 5;

    for url in crawler.get_next_page_urls(category) {
        for page in (1..10000).step_by(concurrent_pages) {
            let mut page_requests = vec![];
            for page in page..page + concurrent_pages {
                let url = url.replace("{page}", (page).to_string().as_ref());

                page_requests.push(get_data(url));
            }

            let next_pages = join_all(page_requests).await;
            let mut all_successful = true;

            let mut current_page = page;
            for response in next_pages {
                if response.is_ok() && false {
                    let parsed = parse_html(response.unwrap(), &mut products, crawler.clone());

                    all_successful = all_successful && parsed;
                } else {
                    println!("request failed: {:?}", response.err());
                    let _result = postpone_page_parsing(ParsePageMessage {
                        url: url.replace("{page}", (current_page).to_string().as_ref()),
                        source: source.clone(),
                        category: category.clone(),
                    }).await;
                }

                current_page = current_page + 1;
            }

            if !all_successful {
                break;
            }
        }
    }

    products.dedup_by(|a, b| {
        if a.external_id == b.external_id && a.price != b.price {
            let message = format!(
                "Warning! Same external_id, different prices. Parser: {}, id: {}, price1: {}, price2: {}",
                crawler.get_source().to_string().to_snake_case(),
                a.external_id,
                a.price.to_string(),
                b.price.to_string()
            );
            sentry::capture_message(message.as_str(), sentry::Level::Warning);
        }

        a.external_id == b.external_id
    });

    add_parse_breadcrumb(
        "parsed",
        btreemap! {
                    "crawler" => crawler.get_source().to_string(),
                    "category" => category.to_string(),
                    "length" => (products.len() - current_length).to_string()
                },
    );

    save_parsed_products(crawler, products, category).await;

    Ok(())
}

async fn save_parsed_products(crawler: &dyn Crawler, products: Vec<ParsedProduct>, category: &CategorySlug) {
    let mut savings_in_progress = vec![];

    for parsed_product in &products {
        savings_in_progress.push(save_parsed_product(crawler.clone(), &parsed_product, category));

        if savings_in_progress.len() == 15 {
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

        if details.is_some() {
            update_details(&product, &details.unwrap());
        }
    }

    link_to_product(&product, parsed_product, crawler.get_source());
}

fn parse_html(data: String, mut products: &mut Vec<ParsedProduct>, crawler: &dyn Crawler) -> bool {
    let document = Html::parse_document(&data);

    crawler.extract_products(&document, &mut products)
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

    if data.is_err() {
        let message = format!(
            "request for additional data failed! {:?} [{}]",
            data.err(),
            crawler.get_source().to_string().to_snake_case()
        );
        sentry::capture_message(message.as_str(), sentry::Level::Warning);

        return None;
    }

    let document = Html::parse_document(&data.unwrap());

    crawler.extract_additional_info(&document, external_id).await
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
use futures::future::*;
use inflector::Inflector;
use maplit::*;
use scraper::Html;
use sentry::{add_breadcrumb, Breadcrumb};
use sentry::protocol::Value;
use sentry::types::protocol::latest::map::BTreeMap;

use crate::db::entity::CategorySlug;
use crate::db::repository::product::{create_if_not_exists, update_details};
use crate::db::repository::source_product::link_to_product;
use crate::parse::crawler::crawler::Crawler;
use crate::parse::parsed_product::{AdditionalParsedProductInfo, ParsedProduct};
use crate::parse::requester::get_data;

pub async fn parse<T: Crawler>(crawler: &T) -> Result<(), reqwest::Error> {
    add_parse_breadcrumb(
        "started",
        btreemap! {
                    "crawler" => crawler.get_source().to_string(),
              },
    );

    for category in crawler.get_categories() {
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

                for response in next_pages {
                    if response.is_ok() {
                        let parsed = parse_html(response.unwrap(), &mut products, crawler);

                        all_successful = all_successful && parsed;
                    } else {
                        // TODO what if one requests failed? like middle one just because of network error
                        println!("request failed: {:?}", response.err());
                    }
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

        let mut savings_in_progress = vec![];

        for parsed_product in &products {
            savings_in_progress.push(save_parsed_product(crawler, &parsed_product, category));

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
                    "length" => (products.len() - current_length).to_string()
                },
        );
    }
    add_parse_breadcrumb(
        "finished",
        btreemap! {
                    "crawler" => crawler.get_source().to_string(),
                },
    );

    Ok(())
}

async fn save_parsed_product<T: Crawler>(crawler: &T, parsed_product: &ParsedProduct, category: &CategorySlug) {
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

fn parse_html<T: Crawler>(data: String, mut products: &mut Vec<ParsedProduct>, crawler: &T) -> bool {
    let document = Html::parse_document(&data);

    crawler.extract_products(&document, &mut products)
}

async fn extract_additional_info<T: Crawler>(external_id: String, crawler: &T) -> Option<AdditionalParsedProductInfo> {
    add_parse_breadcrumb(
        "extracting additional info",
        btreemap! {
                    "crawler" => crawler.get_source().to_string(),
                    "external_id" => external_id.clone()
                },
    );

    let url = crawler.get_additional_info_url(external_id);
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

    crawler.extract_additional_info(&document).await
}

fn add_parse_breadcrumb(message: &str, data: BTreeMap<&str, String>) {
    let mut btree_data = BTreeMap::new();
    for pair in data {
        btree_data.insert(pair.0.to_string(), Value::from(pair.1));
    }

    add_breadcrumb(Breadcrumb {
        category: Some("parse".into()),
        data: btree_data,
        message: Some(message.to_string()),
        ..Default::default()
    });
}
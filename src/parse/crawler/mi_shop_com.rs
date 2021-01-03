use async_trait::async_trait;
use inflector::Inflector;
use regex::Regex;
use scraper::{Html, Selector};

use crate::db::entity::{CategorySlug, SourceName};
use crate::parse::cloud_uploader::upload_image_to_cloud;
use crate::parse::crawler::crawler::Crawler;
use crate::parse::parsed_product::{AdditionalParsedProductInfo, ParsedProduct};

pub struct MiShopComCrawler {}

fn get_base() -> &'static str {
    "https://mi-shop.com"
}

#[async_trait(? Send)]
impl Crawler for MiShopComCrawler {
    fn get_source(&self) -> &SourceName {
        &SourceName::MiShopCom
    }

    fn get_categories(&self) -> Vec<&CategorySlug> {
        vec![
            // &CategorySlug::Smartphone,
            // &CategorySlug::SmartHome,
            // &CategorySlug::Headphones,
            &CategorySlug::Watches,
        ]
    }

    fn get_next_page_urls(&self, category: &CategorySlug) -> Vec<String> {
        let host = get_base().to_string();
        let base = [host, "/ru/catalog/".to_string()].concat();
        let pagination = "/page/{page}/";

        let urls = match category {
            CategorySlug::Smartphone => vec!["smartphones"],
            CategorySlug::SmartHome => vec![
                "smart_devices/umnyy-dom",
                "smart_devices/foto-video",
                "smart_devices/osveshchenie"
            ],
            CategorySlug::Headphones => vec!["audio"],
            CategorySlug::Watches => vec!["smart_devices/umnye-chasy-i-braslety"]
        };

        urls.into_iter().map(|url| {
            [base.clone(), url.to_string(), pagination.to_string()].concat()
        }).collect()
    }

    fn extract_products(&self, document: Html, all_products: &mut Vec<ParsedProduct>) -> bool {
        let items_selector = Selector::parse(".catalog-item").unwrap();

        let title_selector = Selector::parse(".snippet-card__title").unwrap();
        let price_selector = Selector::parse(".snippet-card__price-new").unwrap();
        let available_selector = Selector::parse(".btn-basket.disabled").unwrap();
        let id_selector = Selector::parse("a.snippet-card__media").unwrap();

        let mut amount_of_parsed_products = 0;
        for element in document.select(&items_selector) {
            amount_of_parsed_products = amount_of_parsed_products + 1;
            let mut title: String;
            let price: f64;
            let available: bool;
            let external_id: String;

            let title_node = element.select(&title_selector).next();
            let price_node = element.select(&price_selector).next();
            let unavailable_node = element.select(&available_selector).next();
            let id_node = element.select(&id_selector).next();

            external_id = id_node.unwrap().value().attr("href").unwrap().to_string();
            title = title_node.unwrap().inner_html();
            let price_text = price_node.unwrap()
                .inner_html()
                .replace("₽", "")
                .replace(" ", "")
                .trim()
                .parse::<f64>();

            if price_text.is_ok() {
                price = price_text.unwrap();
            } else {
                println!("Price parsing failed: {}", price_text.err().unwrap()); // TODO error tracking
                continue;
            }

            if title.contains('(') { // TODO different color can cost different price
                title = title.split('(').next().unwrap().trim().to_string()
            }

            available = unavailable_node.is_none();

            all_products.push(ParsedProduct {
                title,
                price,
                available,
                external_id,
            });
        }

        amount_of_parsed_products > 0
    }

    fn get_additional_info_url(&self, external_id: String) -> String {
        format!("{}{}", get_base(), external_id)
    }

    async fn extract_additional_info(&self, document: Html) -> AdditionalParsedProductInfo {
        let images_selector = Selector::parse(".detail__slides img").unwrap();
        let image_nodes = document.select(&images_selector);
        let mut images_urls: Vec<String> = vec![];

        for image in image_nodes.into_iter() {
            let url_path: String;
            let src_tag = image.value().attr("src");
            if src_tag.is_some() {
                url_path = src_tag.unwrap().to_string();
            } else {
                let lazy_tag = image.value().attr("data-lazy");
                url_path = lazy_tag.unwrap().to_string();
            }

            images_urls.push(url_path);
        }

        let mut uploaded_urls: Vec<String> = vec![];
        for image_url in images_urls {
            let filename = [
                "product_images/".to_string(),
                SourceName::MiShopCom.to_string().to_snake_case(),
                image_url.clone()
            ].concat();

            let uploaded = upload_image_to_cloud(
                filename.clone(),
                [get_base().to_string(), image_url.clone()].concat().as_str(),
            ).await;

            if uploaded {
                uploaded_urls.push(filename);
            }
        }

        AdditionalParsedProductInfo {
            image_urls: uploaded_urls,
            description: extract_description(document),
            available: true,
        }
    }
}

fn extract_description(document: Html) -> String {
    lazy_static! {
            static ref RE: Regex = Regex::new(r"(?ms)<p>.*?</p>|<h2>.*?</h2>|<ul>.*?</ul>").unwrap();
        }
    let description_selector = Selector::parse(".detail__tab-description").unwrap();
    let description_node = document.select(&description_selector).next();
    let description: String = description_node.unwrap().inner_html();

    let mut description_sanitized: Vec<&str> = vec![];
    let matches = RE.captures_iter(description.as_str());

    for capture in matches {
        for text in capture.iter() {
            if text.is_some() {
                description_sanitized.push(text.unwrap().as_str());
            }
        }
    }

    if description_sanitized.is_empty() {
        description_sanitized.push(r"<p>");
        description_sanitized.push(description.trim());
        description_sanitized.push(r"<\p>");
    }

    description_sanitized.concat()
}
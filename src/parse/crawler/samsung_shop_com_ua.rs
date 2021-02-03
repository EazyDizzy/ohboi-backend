use async_trait::async_trait;
use regex::Regex;
use scraper::{Html, Selector};

use crate::parse::crawler::crawler::Crawler;
use crate::parse::db::entity::{CategorySlug, SourceName};
use crate::parse::parsed_product::{AdditionalParsedProductInfo, ParsedProduct};

#[derive(Clone)]
pub struct SamsungShopComUaCrawler {}

#[async_trait(? Send)]
impl Crawler for SamsungShopComUaCrawler {
    fn get_source(&self) -> &SourceName { &SourceName::SamsungShopComUa }

    fn get_categories(&self) -> Vec<&CategorySlug> {
        vec![
            &CategorySlug::Watches,
        ]
    }

    fn get_next_page_urls(&self, category: &CategorySlug) -> Vec<String> {
        let host = self.get_base();

        let base = [host, "/ru/".to_string()].concat();
        let pagination = "?page={page}";

        let urls = match category {
            CategorySlug::Watches => vec!["wearables"],
            c => {
                panic!(format!("Unsupported category {}", c));
            }
        };

        urls.into_iter().map(|url| {
            [base.clone(), url.to_string(), pagination.to_string()].concat()
        }).collect()
    }

    fn extract_products(&self, document: &Html) -> Vec<ParsedProduct> {
        let title_re: Regex = Regex::new(r"[a-zA-Z0-9 \-+()]{2,}").unwrap();
        let price_re: Regex = Regex::new(r"[0-9][0-9 ]*[0-9]").unwrap();

        let mut parsed_products = vec![];
        let items_selector = Selector::parse(".catalog-product-item").unwrap();
        let title_selector = Selector::parse(".catalog-product-item_name a").unwrap();
        let price_selector = Selector::parse(".catalog-product-item_price").unwrap();
        let available_selector = Selector::parse(".product-button_buy").unwrap();
        let unavailable_selector = Selector::parse(".product-button_buy.null").unwrap();
        let id_selector = Selector::parse(".catalog-product-item_name a").unwrap();

        for element in document.select(&items_selector) {
            let title: String = {
                let title_node = element.select(&title_selector).next();

                if title_node.is_none() {
                    let message = format!(
                        "title_node not found! [{}]",
                        self.get_source().to_string()
                    );
                    sentry::capture_message(message.as_str(), sentry::Level::Warning);
                    continue;
                }
                let title_value = title_node.unwrap().inner_html();
                let english_text = title_re.find(title_value.as_str()).unwrap();

                english_text.as_str().trim().to_string()
            };

            let price: f64 = {
                let price_node = element.select(&price_selector).next();

                if price_node.is_none() {
                    let message = format!(
                        "price_node not found! [{}]",
                        self.get_source().to_string()
                    );
                    sentry::capture_message(message.as_str(), sentry::Level::Warning);
                    continue;
                }

                let price_html = price_node.unwrap().inner_html();

                let price_text = price_re.find(price_html.as_str()).unwrap()
                                         .as_str().to_string()
                                         .replace(" ", "")
                                         .parse::<f64>();

                if price_text.is_err() {
                    let message = format!(
                        "price_text ({}) can't be parsed! {:?} [{}]",
                        price_node.unwrap().inner_html(),
                        price_text.err(),
                        self.get_source().to_string()
                    );
                    sentry::capture_message(message.as_str(), sentry::Level::Warning);
                    continue;
                }

                price_text.unwrap()
            };

            let available: bool = {
                let available_node = element.select(&available_selector).next();
                let unavailable_node = element.select(&unavailable_selector).next();

                if available_node.is_none() && unavailable_node.is_none() {
                    let message = format!(
                        "both available_node & unavailable_node not found! [{}]",
                        self.get_source()
                    );
                    sentry::capture_message(message.as_str(), sentry::Level::Warning);
                    continue;
                }

                available_node.is_some() && unavailable_node.is_none()
            };

            let external_id: String = {
                let id_node = element.select(&id_selector).next();

                if id_node.is_none() {
                    let message = format!(
                        "id_node not found! [{}]",
                        self.get_source()
                    );
                    sentry::capture_message(message.as_str(), sentry::Level::Warning);
                    continue;
                }

                let id_href = id_node.unwrap().value().attr("href");

                if id_href.is_none() {
                    let message = format!(
                        "id_node doesn't have href! [{}]",
                        self.get_source()
                    );
                    sentry::capture_message(message.as_str(), sentry::Level::Warning);
                    continue;
                }

                id_href.unwrap().to_string()
            };

            println!("{:?}", ParsedProduct {
                title,
                price,
                available,
                external_id,
            });
        }

        parsed_products
    }

    fn get_additional_info_url(&self, external_id: String) -> String {
        unimplemented!()
    }

    async fn extract_additional_info(&self, document: &Html, external_id: String) -> Option<AdditionalParsedProductInfo> {
        unimplemented!()
    }
}

impl SamsungShopComUaCrawler {
    fn get_base(&self) -> String {
        "https://samsungshop.com.ua".to_string()
    }
}
use std::sync::Mutex;

use async_trait::async_trait;
use futures::future::{err, ok};
use futures::future::*;
use futures::FutureExt;
use inflector::Inflector;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use scraper::html::Select;

use crate::parse::cloud_uploader::upload_image_to_cloud;
use crate::parse::consumer::parse_image::UploadImageMessage;
use crate::parse::db::entity::{CategorySlug, SourceName};
use crate::parse::parsed_product::{AdditionalParsedProductInfo, ParsedProduct};
use crate::parse::queue::postpone_image_parsing;

#[async_trait(? Send)]
pub trait Crawler {
    fn get_source(&self) -> &SourceName;

    fn get_categories(&self) -> Vec<&CategorySlug>;

    fn get_next_page_urls(&self, category: &CategorySlug) -> Vec<String>;

    fn extract_products(&self, document: &Html) -> Vec<ParsedProduct>;

    fn get_additional_info_url(&self, external_id: &str) -> String;

    async fn extract_additional_info(&self, document: &Html, external_id: &str) -> Option<AdditionalParsedProductInfo>;

    async fn abstract_extract_images(&self, image_urls: Vec<String>, external_id: &str, base: String) -> Vec<String> {
        let mut uploaded_urls: Vec<String> = vec![];
        let mut uploads: Vec<_> = vec![];

        let upload_later = Mutex::new(vec![]);
        for image_url in image_urls {
            let file_path = [
                "product_images/".to_string(),
                self.get_source().to_string().to_snake_case(),
                image_url.clone()
            ].concat();

            let url: String = [base.clone(), image_url.clone()].concat();

            uploads.push(
                upload_image_to_cloud(file_path.clone(), url.clone(),
                ).then(|success| {
                    if success {
                        ok(file_path)
                    } else {
                        upload_later
                            .lock()
                            .unwrap()
                            .push(UploadImageMessage {
                                file_path: file_path.clone(),
                                image_url: url,
                                external_id: external_id.to_string(),
                                source: *self.get_source(),
                            });
                        err(file_path)
                    }
                })
            );
        }

        let uploaded_images = join_all(uploads).await;

        for filename in uploaded_images {
            if filename.is_ok() {
                uploaded_urls.push(filename.unwrap());
            }
        }

        for message in upload_later.lock().unwrap().to_vec() {
            let _schedule_result = postpone_image_parsing(message).await;
        }

        uploaded_urls
    }

    fn abstract_extract_image_urls(&self, image_nodes: Select, lazy_attribute: &str) -> Vec<String> {
        let mut images_urls: Vec<String> = vec![];

        for image in image_nodes.into_iter() {
            let url_path: String;
            let src_tag = image.value().attr("src");

            if src_tag.is_some() {
                url_path = src_tag.unwrap().to_string();
            } else {
                let lazy_tag = image.value().attr(lazy_attribute);
                if lazy_tag.is_none() {
                    let message = format!(
                        "both src & lazy tags not found! [{source}]",
                        source = self.get_source()
                    );
                    sentry::capture_message(message.as_str(), sentry::Level::Warning);
                    continue;
                }
                url_path = lazy_tag.unwrap().to_string();
            }
            images_urls.push(url_path);
        }

        images_urls
    }

    fn abstract_extract_description(&self, document: &Html, selector: Selector, re: &Regex) -> Option<String> {
        let description_node = document.select(&selector).next();

        if description_node.is_none() {
            let message = format!(
                "description_node not found! [{source}]",
                source = self.get_source()
            );
            sentry::capture_message(message.as_str(), sentry::Level::Warning);

            return None;
        }

        let description: String = description_node.unwrap().inner_html();

        let mut description_sanitized: Vec<&str> = vec![];
        let matches = re.captures_iter(description.as_str());

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

        Some(description_sanitized.concat())
    }

    fn abstract_parse_availability(
        &self,
        document: &Html,
        available_selector: Selector,
        unavailable_selector: Selector,
    ) -> Option<bool> {
        let buy_button_node = document.select(&available_selector).next();
        let unavailable_button_node = document.select(&unavailable_selector).next();

        if buy_button_node.is_none() && unavailable_button_node.is_none() {
            let message = format!(
                "both buy_button_node & unavailable_button_node not found! [{source}]",
                source = self.get_source()
            );
            sentry::capture_message(message.as_str(), sentry::Level::Warning);

            return None;
        }

        Some(buy_button_node.is_some() && unavailable_button_node.is_none())
    }
}

pub fn get_html_nodes<'a>(selectors: &'a ProductHtmlSelectors, element: &'a ElementRef, source: &'a SourceName) -> Option<ProductHtmlNodes<'a>> {
    let id_node = element.select(&selectors.id).next();
    let title_node = element.select(&selectors.title).next();
    let price_node = element.select(&selectors.price).next();
    let available_node = element.select(&selectors.available).next();
    let unavailable_node = element.select(&selectors.unavailable).next();
    let mut valid = true;

    if id_node.is_none() {
        let message = format!("id_node not found! [{source}]", source = source);
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
        valid = false;
    }

    if title_node.is_none() {
        let message = format!("title_node not found! [{source}]", source = source);
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
        valid = false;
    }
    if price_node.is_none() {
        let message = format!("price_node not found! [{source}]", source = source);
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
        valid = false;
    }

    if available_node.is_none() && unavailable_node.is_none() {
        let message = format!("both available_node & unavailable_node not found! [{source}]", source = source);
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
        valid = false;
    }

    if valid {
        Some(ProductHtmlNodes {
            id: id_node.unwrap(),
            title: title_node.unwrap(),
            price: price_node.unwrap(),
            available: available_node,
            unavailable: unavailable_node,
        })
    } else {
        None
    }
}

pub struct ProductHtmlSelectors {
    pub id: Selector,
    pub title: Selector,
    pub price: Selector,
    pub available: Selector,
    pub unavailable: Selector,
}

pub struct ProductHtmlNodes<'a> {
    pub id: ElementRef<'a>,
    pub title: ElementRef<'a>,
    pub price: ElementRef<'a>,
    pub available: Option<ElementRef<'a>>,
    pub unavailable: Option<ElementRef<'a>>,
}
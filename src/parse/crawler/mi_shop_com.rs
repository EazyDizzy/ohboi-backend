use std::sync::Mutex;

use async_trait::async_trait;
use futures::future::*;
use inflector::Inflector;
use regex::Regex;
use scraper::{Html, Selector};

use crate::parse::cloud_uploader::{upload_image_later, upload_image_to_cloud, UploadImageMessage};
use crate::parse::crawler::crawler::Crawler;
use crate::parse::db::entity::{CategorySlug, SourceName};
use crate::parse::parsed_product::{AdditionalParsedProductInfo, ParsedProduct};

pub struct MiShopComCrawler {}

fn get_base() -> String {
    "https://mi-shop.com".to_string()
}

#[async_trait(? Send)]
impl Crawler for MiShopComCrawler {
    fn get_source(&self) -> &SourceName {
        &SourceName::MiShopCom
    }

    fn get_categories(&self) -> Vec<&CategorySlug> {
        vec![
            &CategorySlug::Smartphone,
            &CategorySlug::SmartHome,
            &CategorySlug::Headphones,
            &CategorySlug::Watches,
        ]
    }

    fn get_next_page_urls(&self, category: &CategorySlug) -> Vec<String> {
        let host = get_base();
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

    fn extract_products(&self, document: &Html, all_products: &mut Vec<ParsedProduct>) -> bool {
        let items_selector = Selector::parse(".catalog-item").unwrap();
        let title_selector = Selector::parse(".snippet-card__title").unwrap();
        let price_selector = Selector::parse(".snippet-card__price-new").unwrap();
        let available_selector = Selector::parse(".btn-basket").unwrap();
        let unavailable_selector = Selector::parse(".btn-basket.disabled").unwrap();
        let id_selector = Selector::parse("a.snippet-card__media").unwrap();

        let mut amount_of_parsed_products = 0;
        for element in document.select(&items_selector) {
            amount_of_parsed_products = amount_of_parsed_products + 1;

            let title: String = {
                let title_node = element.select(&title_selector).next();

                if title_node.is_none() {
                    let message = format!(
                        "title_node not found! [{}]",
                        self.get_source().to_string().to_snake_case()
                    );
                    sentry::capture_message(message.as_str(), sentry::Level::Warning);
                    continue;
                }

                let mut title_value = title_node.unwrap().inner_html();
                if title_value.contains('(') {
                    // removing color information from title
                    title_value = title_value.split('(').next().unwrap().trim().to_string();
                }

                title_value
            };

            let price: f64 = {
                let price_node = element.select(&price_selector).next();

                if price_node.is_none() {
                    let message = format!(
                        "price_node not found! [{}]",
                        self.get_source().to_string().to_snake_case()
                    );
                    sentry::capture_message(message.as_str(), sentry::Level::Warning);
                    continue;
                }

                let price_text = price_node.unwrap()
                                           .inner_html()
                                           .replace("₽", "")
                                           .replace(" ", "")
                                           .trim()
                                           .parse::<f64>();

                if price_text.is_err() {
                    let message = format!(
                        "price_text({}) can't be parsed! {:?} [{}]",
                        price_node.unwrap().inner_html(),
                        price_text.err(),
                        self.get_source().to_string().to_snake_case()
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
                        self.get_source().to_string().to_snake_case()
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
                        self.get_source().to_string().to_snake_case()
                    );
                    sentry::capture_message(message.as_str(), sentry::Level::Warning);
                    continue;
                }

                let id_href = id_node.unwrap().value().attr("href");

                if id_href.is_none() {
                    let message = format!(
                        "id_node doesn't have href! [{}]",
                        self.get_source().to_string().to_snake_case()
                    );
                    sentry::capture_message(message.as_str(), sentry::Level::Warning);
                    continue;
                }

                id_href.unwrap().to_string()
            };

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

    async fn extract_additional_info(&self, document: &Html, external_id: String) -> Option<AdditionalParsedProductInfo> {
        let image_urls = self.extract_images(document, external_id).await;
        let description = self.extract_description(document);
        let available = self.parse_availability(document);

        if description.is_none() || available.is_none() {
            return None;
        }

        Some(AdditionalParsedProductInfo {
            image_urls,
            description: description.unwrap(),
            available: available.unwrap(),
        })
    }
}

impl MiShopComCrawler {
    fn extract_description(&self, document: &Html) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?ms)<p>.*?</p>|<h2>.*?</h2>|<ul>.*?</ul>").unwrap();
        }
        let description_selector = Selector::parse(".detail__tab-description").unwrap();
        let description_node = document.select(&description_selector).next();

        if description_node.is_none() {
            let message = format!(
                "description_node not found! [{}]",
                self.get_source().to_string().to_snake_case()
            );
            sentry::capture_message(message.as_str(), sentry::Level::Warning);

            return None;
        }

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

        Some(description_sanitized.concat())
    }

    async fn extract_images(&self, document: &Html, external_id: String) -> Vec<String> {
        let images_urls = self.extract_image_urls(document);

        let mut uploaded_urls: Vec<String> = vec![];
        let mut uploads: Vec<_> = vec![];

        let base = get_base();
        let upload_later = Mutex::new(vec![]);
        for image_url in images_urls {
            let file_path = [
                "product_images/".to_string(),
                SourceName::MiShopCom.to_string().to_snake_case(),
                image_url.clone()
            ].concat();
            let url: String = [base.clone(), image_url.to_string()].concat();

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
                                external_id: external_id.clone(),
                                source: SourceName::MiShopCom,
                            });
                        err(file_path)
                    }
                }));
        }

        let uploaded_images = join_all(uploads).await;

        for filename in uploaded_images {
            if filename.is_ok() {
                uploaded_urls.push(filename.unwrap());
            }
        }
        for message in upload_later.lock().unwrap().to_vec() {
            let _schedule_result = upload_image_later(message).await;
            // TODO what?
        }

        uploaded_urls
    }

    fn extract_image_urls(&self, document: &Html) -> Vec<String> {
        let images_selector = Selector::parse(".detail-modal .detail__slides img").unwrap();
        let image_nodes = document.select(&images_selector);
        let mut images_urls: Vec<String> = vec![];

        for image in image_nodes.into_iter() {
            let url_path: String;
            let src_tag = image.value().attr("src");

            if src_tag.is_some() {
                url_path = src_tag.unwrap().to_string();
            } else {
                let lazy_tag = image.value().attr("data-lazy");
                if lazy_tag.is_none() {
                    let message = format!(
                        "both src & data-lazy tags not found! [{}]",
                        self.get_source().to_string().to_snake_case()
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

    fn parse_availability(&self, document: &Html) -> Option<bool> {
        let buy_button_selector = Selector::parse(".btn-primary.buy-btns__buy").unwrap();
        let unavailable_button_selector = Selector::parse(".btn-primary.detail-subscribe__btn").unwrap();
        let buy_button_node = document.select(&buy_button_selector).next();
        let unavailable_button_node = document.select(&unavailable_button_selector).next();

        if buy_button_node.is_none() && unavailable_button_node.is_none() {
            let message = format!(
                "both buy_button_node & unavailable_button_node not found! [{}]",
                self.get_source().to_string().to_snake_case()
            );
            sentry::capture_message(message.as_str(), sentry::Level::Warning);

            return None;
        }

        Some(buy_button_node.is_some() && unavailable_button_node.is_none())
    }
}


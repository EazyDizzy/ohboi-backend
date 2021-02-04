use async_trait::async_trait;
use regex::Regex;
use scraper::{Html, Selector};

use crate::parse::crawler::crawler::{Crawler, get_html_nodes, ProductHtmlSelectors};
use crate::parse::db::entity::{CategorySlug, SourceName};
use crate::parse::parsed_product::{AdditionalParsedProductInfo, ParsedProduct};

#[derive(Clone)]
pub struct MiShopComCrawler {}

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
        let host = self.get_base();
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

    fn extract_products(&self, document: &Html) -> Vec<ParsedProduct> {
        let mut parsed_products = vec![];
        let items_selector = Selector::parse(".catalog-item").unwrap();

        let selectors = ProductHtmlSelectors {
            id: Selector::parse("a.snippet-card__media[href]").unwrap(),
            title: Selector::parse(".snippet-card__title").unwrap(),
            price: Selector::parse(".snippet-card__price-new").unwrap(),
            available: Selector::parse(".btn-basket").unwrap(),
            unavailable: Selector::parse(".btn-basket.disabled").unwrap(),
        };

        for element in document.select(&items_selector) {
            let nodes = get_html_nodes(&selectors, &element, &self.get_source());

            if nodes.is_none() {
                continue;
            }

            let product_nodes = nodes.unwrap();

            let title: String = {
                let mut title_value = product_nodes.title.inner_html();
                if title_value.contains('(') {
                    // removing color information from title
                    title_value = title_value.split('(').next().unwrap().trim().to_string();
                }

                title_value
            };

            let price: f64 = {
                let price_html = product_nodes.price.inner_html();
                let price_text = price_html
                    .replace("₽", "")
                    .replace(" ", "")
                    .trim()
                    .parse::<f64>();

                if price_text.is_err() {
                    let message = format!(
                        "price_text({}) can't be parsed! {:?} [{}]",
                        price_html,
                        price_text.err(),
                        self.get_source()
                    );
                    sentry::capture_message(message.as_str(), sentry::Level::Warning);
                    continue;
                }

                price_text.unwrap()
            };

            let available = product_nodes.available.is_some() && product_nodes.unavailable.is_none();
            let external_id = product_nodes.id.value().attr("href").unwrap().to_string();

            if title.is_empty() || external_id.is_empty() {
                let message = format!(
                    "Some param is invalid ({}): title - {}, external_id - {}",
                    self.get_source(),
                    title,
                    external_id,
                );
                sentry::capture_message(message.as_str(), sentry::Level::Warning);
                continue;
            }
            parsed_products.push(ParsedProduct {
                title,
                price,
                available,
                external_id,
            });
        }

        parsed_products
    }

    fn get_additional_info_url(&self, external_id: String) -> String {
        format!("{}{}", self.get_base(), external_id)
    }

    async fn extract_additional_info(&self, document: &Html, external_id: String) -> Option<AdditionalParsedProductInfo> {
        let image_urls = self.extract_images(document, external_id).await;
        // TODO replace tags to some standard
        let description = self.abstract_extract_description(
            &document,
            Selector::parse(".detail__tab-description").unwrap(),
            &DESCRIPTION_RE,
        );
        let available = self.abstract_parse_availability(
            document,
            Selector::parse(".btn-primary.buy-btns__buy").unwrap(),
            Selector::parse(".btn-primary.detail-subscribe__btn").unwrap(),
        );

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

lazy_static! {
    static ref DESCRIPTION_RE: Regex = Regex::new(r"(?ms)<p>.*?</p>|<h2>.*?</h2>|<ul>.*?</ul>").unwrap();
}

impl MiShopComCrawler {
    fn get_base(&self) -> String {
        "https://mi-shop.com".to_string()
    }

    async fn extract_images(&self, document: &Html, external_id: String) -> Vec<String> {
        let images_selector = Selector::parse(".detail-modal .detail__slides img").unwrap();
        let image_nodes = document.select(&images_selector);
        let images_urls = self.abstract_extract_image_urls(image_nodes, "data-lazy");

        self.abstract_extract_images(images_urls, external_id, self.get_base()).await
    }
}


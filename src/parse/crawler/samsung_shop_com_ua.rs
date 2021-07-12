use async_trait::async_trait;
use regex::Regex;
use scraper::{Html, Selector};

use crate::my_enum::CurrencyEnum;
use crate::parse::crawler::crawler::{Crawler, get_html_nodes, ProductHtmlSelectors};
use crate::parse::db::entity::{CategorySlug, SourceName};
use crate::parse::parsed_product::{AdditionalParsedProductInfo, LocalParsedProduct};

static SITE_BASE: &str = "https://samsungshop.com.ua";

#[derive(Clone)]
pub struct SamsungShopComUaCrawler {}

#[async_trait(? Send)]
impl Crawler for SamsungShopComUaCrawler {
    fn get_source(&self) -> SourceName { SourceName::SamsungShopComUa }

    fn get_currency(&self) -> CurrencyEnum {
        CurrencyEnum::UAH
    }

    fn get_categories(&self) -> Vec<CategorySlug> {
        vec![
            CategorySlug::Watches,
        ]
    }

    fn get_next_page_urls(&self, category: CategorySlug) -> Vec<String> {
        let base = [SITE_BASE, "/ru/"].concat();
        let pagination = "?page={page}";

        let urls = match category {
            CategorySlug::Watches => vec!["wearables"],
            c => {
                panic!("Unsupported category {}", c);
            }
        };

        urls.into_iter().map(|url| {
            [&base, url, pagination].concat()
        }).collect()
    }

    fn extract_products(&self, document: &Html) -> Vec<LocalParsedProduct> {
        // to not include russian words in title
        let title_re: Regex = Regex::new(r"[a-zA-Z0-9 \-+()]{2,}").unwrap();
        let price_re: Regex = Regex::new(r"[0-9][0-9 ]*[0-9]").unwrap();

        let mut parsed_products = vec![];
        let items_selector = Selector::parse(".catalog-product-item").unwrap();

        let selectors = ProductHtmlSelectors {
            id: Selector::parse(".catalog-product-item_name a[href]").unwrap(),
            title: Selector::parse(".catalog-product-item_name a").unwrap(),
            price: Selector::parse(".catalog-product-item_price").unwrap(),
            available: Selector::parse(".product-button_buy").unwrap(),
            unavailable: Selector::parse(".product-button_buy.null").unwrap(),
        };

        for element in document.select(&items_selector) {
            let nodes = get_html_nodes(&selectors, &element, self.get_source());

            if nodes.is_none() {
                continue;
            }

            let product_nodes = nodes.unwrap();

            let title: String = {
                let title_value = product_nodes.title.inner_html();
                let english_text = title_re.find(title_value.as_str()).unwrap();

                english_text.as_str().trim().to_string()
            };

            let price: f64 = {
                let price_html = product_nodes.price.inner_html();

                let price_text = price_re.find(price_html.as_str()).unwrap()
                    .as_str().to_string()
                    .replace(" ", "")
                    .parse::<f64>();

                if price_text.is_err() {
                    let message = format!(
                        "price_text ({html}) can't be parsed![{source}] {error:?}",
                        html = price_html,
                        source = self.get_source(),
                        error = price_text.err(),
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
                    "Some param is invalid [{source}]: title - {title}, external_id - {id}",
                    source = self.get_source(),
                    title = title,
                    id = external_id,
                );
                sentry::capture_message(message.as_str(), sentry::Level::Warning);
                continue;
            }

            log::info!("{}", title);
            parsed_products.push(LocalParsedProduct {
                title,
                price,
                available,
                external_id,
            });
        }

        parsed_products
    }

    fn get_additional_info_url(&self, external_id: &str) -> String {
        format!("{}{}", SITE_BASE, external_id)
    }

    async fn extract_additional_info(&self, document: &Html, external_id: &str) -> Option<AdditionalParsedProductInfo> {
        let description = self.abstract_extract_description(
            &document,
            Selector::parse(".acardeon-item-content-main").unwrap(),
            &DESCRIPTION_RE,
        );
        let available = self.abstract_parse_availability(
            document,
            Selector::parse(".product-button_buy").unwrap(),
            Selector::parse(".product-button_buy.null").unwrap(),
        );

        if description.is_none() || available.is_none() {
            None
        } else {
            let image_urls = self.extract_images(document, external_id).await;
            Some(AdditionalParsedProductInfo {
                image_urls,
                description: description.unwrap(),
                available: available.unwrap(),
            })
        }
    }
}

lazy_static! {
    static ref DESCRIPTION_RE: Regex = Regex::new(r"(?ms)<big>.*?</big>|<h3>.*?</h3>").unwrap();
}
impl SamsungShopComUaCrawler {
    async fn extract_images(&self, document: &Html, external_id: &str) -> Vec<String> {
        let images_selector = Selector::parse(".sp-slide img.sp-image").unwrap();
        let image_nodes = document.select(&images_selector);
        let images_urls = self.abstract_extract_image_urls(image_nodes, "data-src");

        self.abstract_extract_images(images_urls, external_id, SITE_BASE).await
    }
}
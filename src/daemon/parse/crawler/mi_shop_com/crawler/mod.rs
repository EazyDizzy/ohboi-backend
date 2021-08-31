mod characteristics;
mod media_format;

use std::time::Instant;

use regex::Regex;
use scraper::{Html, Selector};

use crate::daemon::db::entity::category::CategorySlug;
use crate::daemon::db::entity::source::SourceName;
use crate::daemon::dto::parsed_product::{AdditionalParsedProductInfo, LocalParsedProduct};
use crate::daemon::parse::crawler::crawler::{get_html_nodes, Crawler, ProductHtmlSelectors};
use crate::my_enum::CurrencyEnum;
use crate::daemon::parse::crawler::mi_shop_com::crawler::characteristics::extract_characteristics;

static SITE_BASE: &str = "https://mi-shop.com";

lazy_static! {
    static ref DESCRIPTION_RE: Regex =
        Regex::new(r"(?ms)<p>.*?</p>|<h2>.*?</h2>|<ul>.*?</ul>").unwrap();
}

#[derive(Clone)]
pub struct MiShopComCrawler {}

impl Crawler for MiShopComCrawler {
    fn get_site_base(&self) -> String {
        SITE_BASE.to_string()
    }

    fn get_source(&self) -> SourceName {
        SourceName::MiShopCom
    }

    fn get_currency(&self) -> CurrencyEnum {
        CurrencyEnum::RUB
    }

    fn get_categories(&self) -> Vec<CategorySlug> {
        vec![
            CategorySlug::Smartphone,
            CategorySlug::SmartHome,
            CategorySlug::Headphones,
            CategorySlug::Watches,
        ]
    }

    fn get_next_page_urls(&self, category: CategorySlug) -> Vec<String> {
        let base = [SITE_BASE, "/ru/catalog/"].concat();
        let pagination = "/page/{page}/";

        let urls = match category {
            CategorySlug::Smartphone => vec!["smartphones"],
            CategorySlug::SmartHome => vec![
                "smart_devices/umnyy-dom",
                "smart_devices/foto-video",
                "smart_devices/osveshchenie",
            ],
            CategorySlug::Headphones => vec!["audio"],
            CategorySlug::Watches => vec!["smart_devices/umnye-chasy-i-braslety"],
        };

        urls.into_iter()
            .map(|url| [&base, url, pagination].concat())
            .collect()
    }

    fn extract_products(&self, document: &Html) -> Vec<LocalParsedProduct> {
        let mut parsed_products = vec![];
        let items_selector = Selector::parse(".js-catalog-item").unwrap();

        let selectors = ProductHtmlSelectors {
            id: Selector::parse("a.product-card__name[href]").unwrap(),
            title: Selector::parse(".product-card__title").unwrap(),
            price: Selector::parse(".price__new").unwrap(),
            available: Selector::parse(".btn-buy").unwrap(),
            unavailable: Selector::parse(".btn-buy.disabled").unwrap(),
        };

        for element in document.select(&items_selector) {
            let nodes = get_html_nodes(&selectors, &element, self.get_source());

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
                    .replace('₽', "")
                    .replace(' ', "")
                    .trim()
                    .parse::<f64>();

                if price_text.is_err() {
                    let message = format!(
                        "price_text({html}) can't be parsed![{source}] {error:?}",
                        html = price_html,
                        source = self.get_source(),
                        error = price_text.err(),
                    );
                    sentry::capture_message(message.as_str(), sentry::Level::Warning);
                    continue;
                }

                price_text.unwrap()
            };

            let available =
                product_nodes.available.is_some() && product_nodes.unavailable.is_none();
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

    fn extract_additional_info(
        &self,
        document: &Html,
        external_id: &str,
    ) -> Option<AdditionalParsedProductInfo> {
        let description = self.abstract_extract_description(
            &document,
            Selector::parse(".detail__tab-description").unwrap(),
            &DESCRIPTION_RE,
        );
        let available = self.abstract_parse_availability(
            document,
            Selector::parse(".btn-primary.js-buy").unwrap(),
            Selector::parse("#subscribe-product").unwrap(),
        );

        // We should not upload images if it is not valid product
        if description.is_none() || available.is_none() {
            None
        } else {
            let image_urls = self.extract_images(document);
            let start = Instant::now();
            let characteristics = extract_characteristics(&self, &document, external_id);
            let duration = start.elapsed();
            println!(
                "Time elapsed in extract_characteristics() is: {:?}",
                duration
            );
            Some(AdditionalParsedProductInfo {
                image_urls,
                description: description.unwrap(),
                available: available.unwrap(),
                characteristics,
            })
        }
    }
}

impl MiShopComCrawler {
    fn extract_images(&self, document: &Html) -> Vec<String> {
        let images_selector = Selector::parse(".detail-modal .detail__slides img").unwrap();
        let image_nodes = document.select(&images_selector);
        let image_urls = self.abstract_extract_image_urls(image_nodes, "data-lazy");

        image_urls
    }
}
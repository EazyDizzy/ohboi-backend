use scraper::{Html, Selector};

use crate::parse::crawler::crawler::Crawler;
use crate::parse::parsed_product::ParsedProduct;
use crate::db::entity::{CategorySlug, SourceName};

pub struct MiShopComCrawler {}

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
        let base = "https://mi-shop.com/ru/catalog/";
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
            [base, url, pagination].join("")
        }).collect()
    }

    fn extract_products(&self, document: Html, all_products: &mut Vec<ParsedProduct>) {
        let items_selector = Selector::parse(".catalog-item").unwrap();
        let title_selector = Selector::parse(".snippet-card__title").unwrap();
        let price_selector = Selector::parse(".snippet-card__price-new").unwrap();
        let available_selector = Selector::parse(".btn-basket.disabled").unwrap();
        let image_selector = Selector::parse("picture img").unwrap();

        for element in document.select(&items_selector) {
            let mut title: String;
            let price: f64;
            let mut image = "";
            let available: bool;

            let title_node = element.select(&title_selector).next();
            let price_node = element.select(&price_selector).next();
            let unavailable_node = element.select(&available_selector).next();
            let image_node = element.select(&image_selector).next();

            let image_option = image_node.unwrap().value().attr("src");
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

            if image_option.is_some() {
                image = image_option.unwrap();
            }
            available = unavailable_node.is_none();

            all_products.push(ParsedProduct { title, price, available, image_url: image.to_string() });
        }
    }
}
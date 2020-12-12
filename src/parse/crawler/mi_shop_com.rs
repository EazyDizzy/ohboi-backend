use bigdecimal::ToPrimitive;
use scraper::{Html, Selector};

use crate::parse::crawler::crawler::Crawler;
use crate::parse::parsed_product::ParsedProduct;
use crate::db::entity::CategorySlug;

pub struct MiShopComCrawler {}

impl Crawler for MiShopComCrawler {
    fn get_categories(&self) -> Vec<&CategorySlug> {
        vec![
            &CategorySlug::Smartphone,
            &CategorySlug::SmartHome,
            &CategorySlug::Headphones,
            &CategorySlug::Watches,
        ]
    }

    fn get_next_page_url(&self, category: &CategorySlug, current_page: i32) -> String {
        let base = "https://mi-shop.com/ru/catalog/";
        let pagination = "/page/{page}/";

        let url = match category {
            CategorySlug::Smartphone => "smartphones",
            CategorySlug::SmartHome => "smart_devices/umnyy-dom",
            CategorySlug::Headphones => "audio/besprovodnye-naushniki",
            CategorySlug::Watches => "smart_devices/umnye-chasy-i-braslety"
        };

        [base, url, pagination].join("")
            .replace("{page}", (current_page + 1).to_string().as_ref())
    }

    fn extract_products(&self, document: Html, all_products: &mut Vec<ParsedProduct>) {
        let items_selector = Selector::parse(".catalog-item").unwrap();
        let title_selector = Selector::parse(".snippet-card__title").unwrap();
        let price_selector = Selector::parse(".snippet-card__price-new").unwrap();
        let available_selector = Selector::parse(".btn-basket.disabled").unwrap();
        let image_selector = Selector::parse("picture img").unwrap();

        for element in document.select(&items_selector) {
            let mut title: String;
            let price: f32;
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
                .parse::<f32>();

            if price_text.is_ok() {
                price = price_text.unwrap();
            } else {
                println!("Price parsing failed: {}", price_text.err().unwrap()); // TODO error tracking
                continue;
            }

            if title.contains('(') {
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
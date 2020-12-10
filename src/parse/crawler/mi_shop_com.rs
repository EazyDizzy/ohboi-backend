use crate::parse::crawler::crawler::Crawler;
use scraper::{Html, Selector};
use crate::parse::parsed_product::ParsedProduct;
use bigdecimal::ToPrimitive;

pub struct MiShopComCrawler {}

impl Crawler for MiShopComCrawler {
    fn get_next_page_url(&self, current_page: i32) -> String {
        "https://mi-shop.com/ru/catalog/smartphones/page/{page}/"
            .replace("{page}", (current_page + 1).to_string().as_ref())
    }

    fn extract_products(&self, document: Html, all_products: &mut Vec<ParsedProduct>) {
        let items_selector = Selector::parse(".catalog-item").unwrap();
        let title_selector = Selector::parse(".snippet-card__title").unwrap();
        let price_selector = Selector::parse(".snippet-card__price-new").unwrap();
        let available_selector = Selector::parse(".btn-basket.disabled").unwrap();

        for element in document.select(&items_selector) {
            let title_node = element.select(&title_selector).next();
            let price_node = element.select(&price_selector).next();
            let unavailable_node = element.select(&available_selector).next();

            let mut title_text = title_node.unwrap().inner_html();
            let price_text = price_node.unwrap()
                .inner_html()
                .replace("₽", "")
                .replace(" ", "")
                .trim()
                .parse::<f32>();

            let parsed_price;
            if price_text.is_ok() {
                parsed_price = price_text.unwrap();
            } else {
                parsed_price = -1.to_f32().unwrap();
                println!("Price parsing failed: {}", price_text.err().unwrap());
            }

            if title_text.contains('(') {
                title_text = title_text.split('(').next().unwrap().trim().to_string()
            }
            let available = unavailable_node.is_none();

            all_products.push(ParsedProduct { title: title_text, price: parsed_price, available });
        }
    }
}
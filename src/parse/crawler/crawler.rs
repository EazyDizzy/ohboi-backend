use crate::parse::parsed_product::ParsedProduct;
use scraper::Html;

pub trait Crawler {
    fn get_next_page_url(&self, current_page: i32) -> String;

    fn extract_products(&self, document: Html, all_products: &mut Vec<ParsedProduct>);
}
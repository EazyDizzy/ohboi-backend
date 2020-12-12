use crate::parse::parsed_product::ParsedProduct;
use scraper::Html;
use crate::db::entity::CategorySlug;

pub trait Crawler {
    fn get_categories(&self) -> Vec<&CategorySlug>;

    fn get_next_page_urls(&self, category: &CategorySlug) -> Vec<String>;

    fn extract_products(&self, document: Html, all_products: &mut Vec<ParsedProduct>);
}
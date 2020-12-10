use crate::parse::parsed_product::ParsedProduct;
use scraper::Html;
use crate::db::entity::CategorySlug;

pub trait Crawler {
    fn get_categories(&self) -> Vec<&CategorySlug>;

    fn get_next_page_url(&self, category: &CategorySlug, current_page: i32) -> String; // TODO multiple urls for one category

    fn extract_products(&self, document: Html, all_products: &mut Vec<ParsedProduct>);
}
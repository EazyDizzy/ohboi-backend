use crate::parse::parsed_product::{ParsedProduct, AdditionalParsedProductInfo};
use scraper::Html;
use crate::db::entity::{CategorySlug, SourceName};

pub trait Crawler {
    fn get_source(&self) -> &SourceName;

    fn get_categories(&self) -> Vec<&CategorySlug>;

    fn get_next_page_urls(&self, category: &CategorySlug) -> Vec<String>;

    fn extract_products(&self, document: Html, all_products: &mut Vec<ParsedProduct>) -> bool;

    fn get_additional_info_url(&self, external_id: String) -> String;

    fn extract_additional_info(&self, document: Html) -> AdditionalParsedProductInfo;
}
use scraper::Html;
use async_trait::async_trait;

use crate::db::entity::{CategorySlug, SourceName};
use crate::parse::parsed_product::{AdditionalParsedProductInfo, ParsedProduct};

#[async_trait(?Send)]
pub trait Crawler {
    fn get_source(&self) -> &SourceName;

    fn get_categories(&self) -> Vec<&CategorySlug>;

    fn get_next_page_urls(&self, category: &CategorySlug) -> Vec<String>;

    fn extract_products(&self, document: &Html, all_products: &mut Vec<ParsedProduct>) -> bool;

    fn get_additional_info_url(&self, external_id: String) -> String;

    async fn extract_additional_info(&self, document: &Html) -> Option<AdditionalParsedProductInfo>;
}
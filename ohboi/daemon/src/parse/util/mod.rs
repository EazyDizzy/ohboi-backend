use scraper::Html;

use crate::dto::parsed_product::LocalParsedProduct;
use crate::parse::crawler::Crawler;

pub mod dedup;
pub mod iter;

pub fn parse_html(data: &str, crawler: &dyn Crawler) -> Vec<LocalParsedProduct> {
    let document = Html::parse_document(data);

    crawler.extract_products(&document)
}

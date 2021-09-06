use scraper::Html;

use lib::local_sentry::add_category_breadcrumb;
use crate::parse::crawler::Crawler;
use crate::dto::parsed_product::LocalParsedProduct;
use std::collections::BTreeMap;

pub mod dedup;

pub fn parse_html(data: &str, crawler: &dyn Crawler) -> Vec<LocalParsedProduct> {
    let document = Html::parse_document(data);

    crawler.extract_products(&document)
}

pub fn add_parse_breadcrumb(message: &str, data: BTreeMap<&str, String>) {
    add_category_breadcrumb(message, data, "[daemon]".into());
}

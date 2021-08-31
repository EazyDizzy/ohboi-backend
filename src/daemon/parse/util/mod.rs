use scraper::Html;
use sentry::types::protocol::latest::map::BTreeMap;

use crate::local_sentry::add_category_breadcrumb;
use crate::daemon::crawler::crawler::Crawler;
use crate::daemon::dto::parsed_product::LocalParsedProduct;

pub mod dedup;

pub fn parse_html(data: &str, crawler: &dyn Crawler) -> Vec<LocalParsedProduct> {
    let document = Html::parse_document(data);

    crawler.extract_products(&document)
}

pub fn add_parse_breadcrumb(message: &str, data: BTreeMap<&str, String>) {
    add_category_breadcrumb(message, data, "[daemon]".into());
}

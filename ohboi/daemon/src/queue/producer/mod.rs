use std::collections::BTreeMap;

use lib::local_sentry::add_category_breadcrumb;

pub mod parse_category;
pub mod pull_exchange_rates;

fn add_producer_breadcrumb(message: &str, data: BTreeMap<&str, String>, producer_name: &str) {
    add_category_breadcrumb(message, data, ["producer.", producer_name].join(""));
}

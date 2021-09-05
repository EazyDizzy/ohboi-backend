use maplit::btreemap;

use crate::daemon::parse::crawler::get_crawler;
use crate::daemon::db::entity::category::CategorySlug;
use crate::daemon::db::entity::source::SourceName;
use crate::daemon::parse::layer::save::save_parsed_products;
use crate::daemon::parse::util::dedup::dedup_products;
use crate::daemon::parse::util::{add_parse_breadcrumb, parse_html};
use crate::daemon::service::request::get;

pub async fn parse_category_page(
    url: &str,
    source: SourceName,
    category: CategorySlug,
) -> Result<(), reqwest::Error> {
    let crawler = get_crawler(&source);
    add_parse_breadcrumb(
        "in progress",
        btreemap! {
            "crawler" => source.to_string(),
            "category" => category.to_string(),
        },
    );

    let response = get(url).await?;
    let mut products = parse_html(&response, crawler);

    dedup_products(&mut products, source);

    add_parse_breadcrumb(
        "parsed",
        btreemap! {
            "crawler" => source.to_string(),
            "category" => category.to_string(),
            "length" => products.len().to_string()
        },
    );

    save_parsed_products(
        crawler.get_source(),
        crawler.get_currency(),
        products,
        category,
    )
    .await;

    Ok(())
}

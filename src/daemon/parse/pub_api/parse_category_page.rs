use crate::daemon::db::entity::source::SourceName;
use crate::daemon::db::entity::category::CategorySlug;
use crate::daemon::crawler::get_crawler;
use crate::daemon::service::request::pub_api::get_data;
use maplit::btreemap;
use crate::daemon::parse::layer::save::save_parsed_products;
use crate::daemon::parse::util::dedup::dedup_products;
use crate::daemon::parse::util::{parse_html, add_parse_breadcrumb};

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

    let response = get_data(url).await?;
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
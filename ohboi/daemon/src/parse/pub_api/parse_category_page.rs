use crate::db::entity::category::CategorySlug;
use crate::db::entity::source::SourceName;
use crate::parse::crawler::get_crawler;
use crate::parse::layer::save::save_parsed_products;
use crate::parse::util::dedup::dedup_products;
use crate::parse::util::parse_html;
use crate::service::request::get;

pub async fn parse_category_page(
    url: &str,
    source: SourceName,
    category: CategorySlug,
) -> Result<(), reqwest::Error> {
    let crawler = get_crawler(&source);

    let response = get(url).await?;
    let mut products = parse_html(&response, crawler);

    dedup_products(&mut products, source);

    save_parsed_products(
        crawler.get_source(),
        crawler.get_currency(),
        products,
        category,
    )
    .await;

    Ok(())
}

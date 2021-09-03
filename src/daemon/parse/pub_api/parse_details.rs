use maplit::btreemap;
use scraper::Html;

use crate::daemon::parse::crawler::crawler::Crawler;
use crate::daemon::dto::parsed_product::AdditionalParsedProductInfo;
use crate::daemon::service::request::pub_api::get;
use crate::daemon::parse::util::add_parse_breadcrumb;

pub async fn parse_details(
    external_id: &str,
    crawler: &dyn Crawler,
) -> Option<AdditionalParsedProductInfo> {
    add_parse_breadcrumb(
        "[parse_details] extracting additional info",
        btreemap! {
            "crawler" => crawler.get_source().to_string(),
            "external_id" => external_id.to_string()
        },
    );

    let url = crawler.get_additional_info_url(&external_id);
    let data = get(&url).await;

    match data {
        Ok(data) => {
            let document = Html::parse_document(&data);

            crawler.extract_additional_info(&document, &external_id)
        }
        Err(e) => {
            let message = format!(
                "[parse_details] Request for additional data failed! [{source}] {error:?}",
                source = crawler.get_source().to_string(),
                error = e,
            );
            sentry::capture_message(message.as_str(), sentry::Level::Warning);

            None
        }
    }
}

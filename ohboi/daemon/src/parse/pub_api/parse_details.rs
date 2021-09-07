use scraper::Html;

use lib::error_reporting;
use lib::error_reporting::ReportingContext;

use crate::dto::parsed_product::AdditionalParsedProductInfo;
use crate::parse::crawler::Crawler;
use crate::service::request::get;
use crate::ConsumerName;

pub async fn parse_details(
    external_id: &str,
    crawler: &dyn Crawler,
) -> Option<AdditionalParsedProductInfo> {
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
            error_reporting::error(
                message.as_str(),
                &ReportingContext {
                    executor: &ConsumerName::ParseDetails,
                    action: "parse_details",
                },
            );

            None
        }
    }
}

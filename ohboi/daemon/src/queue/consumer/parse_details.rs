use serde::{Deserialize, Serialize};

use crate::db::entity::source::SourceName;
use crate::db::repository::product::update_details;
use crate::parse::crawler::upload_extracted_images;
use crate::parse::crawler::get_crawler;
use crate::parse::parse_details;
use crate::queue::layer::consume::consume;
use crate::{SETTINGS, ConsumerName};
use lib::error_reporting;
use lib::error_reporting::ReportingContext;

#[derive(Serialize, Deserialize)]
pub struct ParseDetailsMessage {
    pub external_id: String,
    pub source: SourceName,
    pub product_id: i32,
}

pub async fn start() -> core::result::Result<(), ()> {
    let _ = consume(&SETTINGS.queue_broker.queues.parse_details, execute)
        .await
        .expect("Can't launch consumer");

    Ok(())
}

async fn execute(message: ParseDetailsMessage) -> Result<(), ()> {
    let crawler = get_crawler(&message.source);
    let details = parse_details(&message.external_id, crawler).await;

    match details {
        None => {
            error_reporting::error(
                format!(
                    "[parse_details] No additional info found [{source}] for: {id}",
                    source = crawler.get_source().to_string(),
                    id = &message.external_id
                )
                .as_str(),
                &ReportingContext {
                    executor: &ConsumerName::ParseDetails,
                    action: "execute"
                }
            );

            Err(())
        }
        Some(mut details) => {
            let uploaded_urls = upload_extracted_images(
                crawler.get_source(),
                details.image_urls,
                &message.external_id,
                &crawler.get_site_base(),
            )
            .await;
            details.image_urls = uploaded_urls;

            update_details(message.product_id, &details);

            Ok(())
        }
    }
}

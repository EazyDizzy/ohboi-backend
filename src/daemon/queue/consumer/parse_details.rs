use crossbeam::channel;
use serde::{Deserialize, Serialize};
use tokio::runtime::Handle;

use crate::daemon::crawler::crawler::upload_extracted_images;
use crate::daemon::crawler::get_crawler;
use crate::daemon::db::entity::source::SourceName;
use crate::daemon::db::repository::product::update_details;
use crate::daemon::queue::layer::consume::consume;
use crate::daemon::parse::pub_api::parse_details::parse_details;
use crate::SETTINGS;

#[derive(Serialize, Deserialize)]
pub struct ParseDetailsMessage {
    pub external_id: String,
    pub source: SourceName,
    pub product_id: i32,
}

pub async fn start() -> core::result::Result<(), ()> {
    let _ = consume(&SETTINGS.queue_broker.queues.parse_details, |message| {
        let (snd, rcv) = channel::bounded(1);

        let _ = Handle::current().spawn(async move {
            let message: ParseDetailsMessage =
                serde_json::from_str(&message).expect("Failed to parse ParseDetailsMessage");

            let rs = execute(message).await;
            let _ = snd.send(rs);
        });

        rcv.recv()
            .expect("Failed to receive result of thread execution")
    })
    .await
    .expect("Can't launch consumer");

    Ok(())
}

async fn execute(message: ParseDetailsMessage) -> Result<(), ()> {
    let crawler = get_crawler(&message.source);
    let details = parse_details(&message.external_id, crawler).await;

    match details {
        None => {
            sentry::capture_message(
                format!(
                    "No additional info found [{source}] for: {id}",
                    source = crawler.get_source().to_string(),
                    id = &message.external_id
                )
                .as_str(),
                sentry::Level::Warning,
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

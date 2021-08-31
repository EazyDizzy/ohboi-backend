use crossbeam::channel;
use serde::{Deserialize, Serialize};
use tokio::runtime::Handle;

use crate::parse::consumer::layer::consume::consume;
use crate::parse::crawler::crawler::upload_extracted_images;
use crate::parse::db::entity::source::SourceName;
use crate::parse::db::repository::product::update_details;
use crate::parse::service::parser::{extract_additional_info, get_crawler};
use crate::SETTINGS;

#[derive(Serialize, Deserialize)]
pub struct ParseDetailsMessage {
    pub external_id: String,
    pub source: SourceName,
    pub product_id: i32,
}

pub async fn start() -> core::result::Result<(), ()> {
    let _ = consume(&SETTINGS.amqp.queues.parse_details, |message| {
        let (snd, rcv) = channel::bounded(1);

        let _ = Handle::current().spawn(async move {
            let message: ParseDetailsMessage = serde_json::from_str(&message).unwrap();

            let rs = execute(message).await;
            let _ = snd.send(rs);
        });

        rcv.recv().unwrap()
    })
    .await;

    Ok(())
}

async fn execute(message: ParseDetailsMessage) -> Result<(), ()> {
    let crawler = get_crawler(&message.source);
    let details = extract_additional_info(&message.external_id, crawler).await;

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

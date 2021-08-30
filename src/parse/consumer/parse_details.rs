use std::str;

use futures::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions, BasicQosOptions},
    types::FieldTable,
    Result,
};
use maplit::btreemap;
use sentry::protocol::map::BTreeMap;
use serde::{Deserialize, Serialize};

use crate::local_sentry::add_category_breadcrumb;
use crate::parse::crawler::crawler::upload_extracted_images;
use crate::parse::db::entity::source::SourceName;
use crate::parse::db::repository::product::update_details;
use crate::parse::queue::get_channel;
use crate::parse::service::parser::{extract_additional_info, get_crawler};
use crate::SETTINGS;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParseDetailsMessage {
    pub external_id: String,
    pub source: SourceName,
    pub product_id: i32,
}

pub async fn start() -> Result<()> {
    let channel = get_channel().await?;
    channel
        .basic_qos(
            SETTINGS.amqp.queues.parse_details.prefetch,
            BasicQosOptions { global: true },
        )
        .await?;

    let mut consumer = channel
        .basic_consume(
            &SETTINGS.amqp.queues.parse_details.name,
            [&SETTINGS.amqp.queues.parse_details.name, "_consumer"]
                .join("")
                .as_str(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(delivery) = consumer.next().await {
        let (_, delivery) = delivery.expect("error in consumer");

        add_consumer_breadcrumb("got message", btreemap! {});

        let data = str::from_utf8(&delivery.data).unwrap();
        let message: ParseDetailsMessage = serde_json::from_str(data).unwrap();

        // TODO create separate job to attach this info
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

                // TODO requeue with delay https://blog.rabbitmq.com/posts/2015/04/scheduling-messages-with-rabbitmq
                delivery
                    .nack(BasicNackOptions {
                        requeue: true,
                        multiple: false,
                    })
                    .await
                    .expect("nack");
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

                delivery
                    .ack(BasicAckOptions { multiple: false })
                    .await
                    .expect("ack");
            }
        }
    }

    Ok(())
}

fn add_consumer_breadcrumb(message: &str, data: BTreeMap<&str, String>) {
    add_category_breadcrumb(
        message,
        data,
        ["consumer.", &SETTINGS.amqp.queues.parse_details.name].join(""),
    );
}

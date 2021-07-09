use futures::StreamExt;
use lapin::{options::*, Result, types::FieldTable};
use maplit::*;
use sentry::protocol::map::BTreeMap;
use serde::{Deserialize, Serialize};

use crate::local_sentry::add_category_breadcrumb;
use crate::parse::db::entity::{CategorySlug, SourceName};
use crate::parse::service::parser::parse_page;
use crate::parse::queue::get_channel;
use crate::SETTINGS;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParsePageMessage {
    pub url: String,
    pub source: SourceName,
    pub category: CategorySlug,
}

pub async fn start() -> Result<()> {
    let channel = get_channel().await?;
    channel.basic_qos(
        SETTINGS.amqp.queues.parse_page.prefetch,
        BasicQosOptions { global: true },
    ).await?;

    let mut consumer = channel
        .basic_consume(
            &SETTINGS.amqp.queues.parse_page.name,
            [&SETTINGS.amqp.queues.parse_page.name, "_consumer"].join("").as_str(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(delivery) = consumer.next().await {
        let (_, delivery) = delivery.expect("error in consumer");

        let decoded_data = String::from_utf8(delivery.data.clone());
        let data = decoded_data.unwrap();

        let parsed_json = serde_json::from_str(data.as_str());
        let message: ParsePageMessage = parsed_json.unwrap();

        add_consumer_breadcrumb(
            "got message",
            btreemap! {
                     "category" => message.category.to_string(),
                     "source" => message.source.to_string(),
                     "url" => message.url.to_string()
                },
        );

        let parse_result = parse_page(&message.url, &message.source, &message.category).await;

        if parse_result.is_err() {
            let message = format!(
                "Page parsing failed! [{source}]({category}){error:?}",
                source = message.source,
                category = message.category,
                error = parse_result.err()
            );
            sentry::capture_message(message.as_str(), sentry::Level::Warning);
            delivery.nack(BasicNackOptions { requeue: true, multiple: false }).await.expect("nack");
        } else {
            delivery.ack(BasicAckOptions { multiple: false }).await.expect("ack");
        }
    }

    Ok(())
}

fn add_consumer_breadcrumb(message: &str, data: BTreeMap<&str, String>) {
    add_category_breadcrumb(
        message,
        data,
        ["consumer.", &SETTINGS.amqp.queues.parse_page.name].join("").into(),
    );
}
use futures::StreamExt;
use lapin::{options::*, Result, types::FieldTable};
use maplit::*;
use sentry::protocol::map::BTreeMap;

use crate::local_sentry::add_category_breadcrumb;
use crate::parse::service::parser::parse_category;
use crate::parse::producer::parse_category::CrawlerCategoryMessage;
use crate::parse::queue::get_channel;
use crate::SETTINGS;

pub async fn start() -> Result<()> {
    let channel = get_channel().await?;
    channel.basic_qos(
        SETTINGS.amqp.queues.parse_category.prefetch,
        BasicQosOptions { global: true },
    ).await?;

    let mut consumer = channel
        .basic_consume(
            &SETTINGS.amqp.queues.parse_category.name,
            [&SETTINGS.amqp.queues.parse_category.name, "_consumer"].join("").as_str(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(delivery) = consumer.next().await {
        let (_, delivery) = delivery.expect("error in consumer");

        add_consumer_breadcrumb(
            "got message",
            btreemap! {},
        );

        // TODO why clone?
        let decoded_data = String::from_utf8(delivery.data.clone());
        let data = decoded_data.unwrap();

        let parsed_json = serde_json::from_str(data.as_str());
        let message: CrawlerCategoryMessage = parsed_json.unwrap();

        let parse_result = parse_category(&message.source, &message.category).await;

        if parse_result.is_err() {
            let message = format!(
                "Parsing failed! [{source}]({category}) {error:?}",
                error = parse_result.err(),
                source = message.source,
                category = message.category
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
        ["consumer.", &SETTINGS.amqp.queues.parse_category.name].join("").into(),
    );
}
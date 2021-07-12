use std::str;

use futures::StreamExt;
use lapin::{options::*, Result, types::FieldTable};
use maplit::btreemap;
use sentry::protocol::map::BTreeMap;
use serde::{Deserialize, Serialize};

use crate::local_sentry::add_category_breadcrumb;
use crate::parse::db::entity::SourceName;
use crate::parse::db::repository::product::add_image_to_product_details;
use crate::parse::db::repository::source_product::get_by_source_and_external_id;
use crate::parse::queue::get_channel;
use crate::parse::service::cloud_uploader::upload_image_to_cloud;
use crate::SETTINGS;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UploadImageMessage {
    pub file_path: String,
    pub image_url: String,
    pub external_id: String,
    pub source: SourceName,
}

pub async fn start() -> Result<()> {
    let channel = get_channel().await?;
    channel.basic_qos(
        SETTINGS.amqp.queues.parse_image.prefetch,
        BasicQosOptions { global: true },
    ).await?;

    let mut consumer = channel
        .basic_consume(
            &SETTINGS.amqp.queues.parse_image.name,
            [&SETTINGS.amqp.queues.parse_image.name, "_consumer"].join("").as_str(),
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

        let data = str::from_utf8(&delivery.data).unwrap();
        let message: UploadImageMessage = serde_json::from_str(data).unwrap();

        let result = upload_image_to_cloud(message.file_path.clone(), message.image_url).await;

        if result {
            let source_product = get_by_source_and_external_id(&message.source, message.external_id).unwrap();
            add_consumer_breadcrumb(
                "updating product",
                btreemap! {
                    "id" => source_product.product_id.to_string(),
                },
            );
            add_image_to_product_details(source_product.product_id, &message.file_path);

            delivery.ack(BasicAckOptions { multiple: false }).await.expect("ack");
        } else {
            // TODO requeue with delay
            delivery.nack(BasicNackOptions { requeue: true, multiple: false }).await.expect("nack");
        }
    }

    Ok(())
}

fn add_consumer_breadcrumb(message: &str, data: BTreeMap<&str, String>) {
    add_category_breadcrumb(
        message,
        data,
        ["consumer.", &SETTINGS.amqp.queues.parse_image.name].join(""),
    );
}
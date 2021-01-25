use futures::StreamExt;
use lapin::{Connection, ConnectionProperties, options::*, Result, types::FieldTable};
use maplit::*;
use sentry::{add_breadcrumb, Breadcrumb};
use sentry::protocol::map::BTreeMap;
use sentry::protocol::Value;

use crate::parse::crawler::mi_shop_com::MiShopComCrawler;
use crate::parse::db::entity::SourceName;
use crate::parse::parser::parse;
use crate::parse::producer::crawler_category::CrawlerCategoryMessage;
use crate::SETTINGS;

pub async fn start() -> Result<()> {
    let address = &SETTINGS.amqp.url;
    let conn = Connection::connect(
        &address,
        ConnectionProperties::default(),
    )
        .await?;

    let channel = conn.create_channel().await?;
    channel.basic_qos(
        SETTINGS.amqp.queues.crawler_category.prefetch,
        BasicQosOptions { global: true },
    ).await?;

    let mut consumer = channel
        .basic_consume(
            &SETTINGS.amqp.queues.crawler_category.name,
            "crawler_category_consumer",
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

        let decoded_data = String::from_utf8(delivery.data.clone());

        if decoded_data.is_err() {
            let message = format!(
                "Can't decode payload to string! {:?}",
                decoded_data.err()
            );
            sentry::capture_message(message.as_str(), sentry::Level::Warning);
            delivery.nack(BasicNackOptions { requeue: true, multiple: false }).await.expect("nack");
            continue;
        }

        let data = decoded_data.unwrap();
        let parsed_json = serde_json::from_str(data.as_str());
        if parsed_json.is_err() {
            let message = format!(
                "Can't decode json from string! {:?}",
                data.clone()
            );
            sentry::capture_message(message.as_str(), sentry::Level::Warning);
            delivery.nack(BasicNackOptions { requeue: true, multiple: false }).await.expect("nack");
            continue;
        }
        let message: CrawlerCategoryMessage = parsed_json.unwrap();

        add_consumer_breadcrumb(
            "parsed message",
            btreemap! {},
        );
        let crawler = match message.source {
            SourceName::MiShopCom => {
                MiShopComCrawler {}
            }
        };

        let parse_result = parse(&crawler, &message.category).await;

        if parse_result.is_err() {
            let message = format!(
                "Parsing failed! {:?} {} {}",
                parse_result.err(),
                message.source,
                message.category
            );
            sentry::capture_message(message.as_str(), sentry::Level::Warning);
            delivery.nack(BasicNackOptions { requeue: true, multiple: false }).await.expect("nack");
        } else {
            delivery.ack(BasicAckOptions { multiple: false }).await.expect("ack");
        }
    }

    Ok(())
}

pub fn add_consumer_breadcrumb(message: &str, data: BTreeMap<&str, String>) {
    let mut btree_data = BTreeMap::new();

    for pair in data {
        btree_data.insert(pair.0.to_string(), Value::from(pair.1));
    }

    add_breadcrumb(Breadcrumb {
        category: Some("consumer.crawler_category".into()),
        data: btree_data,
        message: Some(message.to_string()),
        ..Default::default()
    });
}
use futures::StreamExt;
use lapin::{options::*, Result, types::FieldTable};
use maplit::*;
use sentry::protocol::map::BTreeMap;

use crate::local_sentry::add_category_breadcrumb;
use crate::parse::crawler::mi_shop_com::MiShopComCrawler;
use crate::parse::db::entity::SourceName;
use crate::parse::parser::parse;
use crate::parse::producer::crawler_category::CrawlerCategoryMessage;
use crate::parse::queue::get_channel;
use crate::SETTINGS;

pub async fn start() -> Result<()> {
    let channel = get_channel().await?;
    channel.basic_qos(
        SETTINGS.amqp.queues.crawler_category.prefetch,
        BasicQosOptions { global: true },
    ).await?;

    let mut consumer = channel
        .basic_consume(
            &SETTINGS.amqp.queues.crawler_category.name,
            [&SETTINGS.amqp.queues.crawler_category.name, "_consumer"].join("").as_str(),
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
        let data = decoded_data.unwrap();

        let parsed_json = serde_json::from_str(data.as_str());
        let message: CrawlerCategoryMessage = parsed_json.unwrap();

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

fn add_consumer_breadcrumb(message: &str, data: BTreeMap<&str, String>) {
    add_category_breadcrumb(
        message,
        data,
        ["consumer.", &SETTINGS.amqp.queues.crawler_category.name].join("").into(),
    );
}
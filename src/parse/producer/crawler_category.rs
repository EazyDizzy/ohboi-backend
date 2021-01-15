use lapin::{BasicProperties, Connection, ConnectionProperties, options::*, Result, types::FieldTable};
use log::info;
use maplit::*;
use sentry::{add_breadcrumb, Breadcrumb};
use sentry::protocol::map::BTreeMap;
use sentry::protocol::Value;

use crate::parse::crawler::crawler::Crawler;
use crate::parse::crawler::mi_shop_com::MiShopComCrawler;

pub async fn start() -> Result<()> {
    let address = std::env::var("AMQP_ADDR").expect("AMQP_ADDR should be set");

    let conn = Connection::connect(
        &address,
        ConnectionProperties::default().with_default_executor(8),
    )
        .await?;

    let crawlers = vec![
        MiShopComCrawler {},
    ];
    let channel = conn.create_channel().await?;
    let queue = channel
        .queue_declare(
            "crawler_category",
            QueueDeclareOptions {
                passive: false,
                durable: true,
                exclusive: false,
                auto_delete: false,
                nowait: false,
            },
            FieldTable::default(),
        )
        .await?;

    for crawler in crawlers {
        for category in crawler.get_categories() {
            let payload = hashmap! {
            "category" => category.to_string(),
            "source" => crawler.get_source().to_string()
        };
            add_producer_breadcrumb(
                "creating",
                btreemap! {
                    "category" => category.to_string(),
                    "source" => crawler.get_source().to_string()
                },
            );

            let payload_json = serde_json::to_string(&payload);

            if payload_json.is_err() {
                let message = format!(
                    "Can't transform payload to json! {:?} [{:?}]",
                    payload_json.err(),
                    payload
                );
                sentry::capture_message(message.as_str(), sentry::Level::Warning);
                continue;
            }

            let confirm = channel
                .basic_publish(
                    "",
                    queue.name().as_str(),
                    BasicPublishOptions::default(),
                    payload_json.unwrap().into_bytes(),
                    BasicProperties::default(),
                )
                .await?
                .await?;

            if confirm.is_nack() {
                // TODO what?
                let message = format!(
                    "Message is not acknowledged!"
                );
                sentry::capture_message(message.as_str(), sentry::Level::Warning);
            } else {
                info!("Message acknowledged");
            }
        }
    }

    Ok(())
}

fn add_producer_breadcrumb(message: &str, data: BTreeMap<&str, String>) {
    let mut btree_data = BTreeMap::new();

    for pair in data {
        btree_data.insert(pair.0.to_string(), Value::from(pair.1));
    }

    add_breadcrumb(Breadcrumb {
        category: Some("producer.crawler_category".into()),
        data: btree_data,
        message: Some(message.to_string()),
        ..Default::default()
    });
}
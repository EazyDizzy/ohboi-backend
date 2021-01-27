use lapin::{BasicProperties, Result};
use lapin::options::BasicPublishOptions;
use maplit::*;
use sentry::{add_breadcrumb, Breadcrumb};
use sentry::protocol::map::BTreeMap;
use sentry::protocol::Value;
use serde::{Deserialize, Serialize};

use crate::parse::crawler::crawler::Crawler;
use crate::parse::crawler::mi_shop_com::MiShopComCrawler;
use crate::parse::db::entity::{CategorySlug, SourceName};
use crate::parse::queue::get_channel;
use crate::SETTINGS;

#[derive(Serialize, Deserialize, Debug)]
pub struct CrawlerCategoryMessage {
    pub category: CategorySlug,
    pub source: SourceName,
}

pub async fn start() -> Result<()> {
    let crawlers = [
        MiShopComCrawler {},
    ];

    let channel = get_channel().await?;

    for crawler in crawlers.iter() {
        for category in crawler.get_categories() {
            let payload = CrawlerCategoryMessage {
                category: *category,
                source: *crawler.get_source(),
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
                    &SETTINGS.amqp.queues.parse_category.name,
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
                log::info!("Message acknowledged");
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
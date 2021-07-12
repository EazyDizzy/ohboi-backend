use lapin::{BasicProperties, Channel, Result};
use lapin::options::BasicPublishOptions;
use maplit::btreemap;
use sentry::protocol::map::BTreeMap;
use serde::{Deserialize, Serialize};

use crate::local_sentry::add_category_breadcrumb;
use crate::parse::crawler::crawler::Crawler;
use crate::parse::crawler::mi_shop_com::MiShopComCrawler;
use crate::parse::crawler::samsung_shop_com_ua::SamsungShopComUaCrawler;
use crate::parse::db::entity::{CategorySlug, SourceName};
use crate::parse::queue::get_channel;
use crate::SETTINGS;

#[derive(Serialize, Deserialize, Debug)]
pub struct CrawlerCategoryMessage {
    pub category: CategorySlug,
    pub source: SourceName,
}

pub async fn start() -> Result<()> {
    let channel = get_channel().await?;
    start_crawler(MiShopComCrawler {}, &channel).await?;
    start_crawler(SamsungShopComUaCrawler {}, &channel).await?;

    Ok(())
}

async fn start_crawler<T>(crawler: T, channel: &Channel) -> Result<()>
    where T: Crawler {

    // TODO check if crawler is enabled
    for category in crawler.get_categories() {
        let payload = CrawlerCategoryMessage {
            category,
            source: crawler.get_source(),
        };
        add_producer_breadcrumb(
            "creating",
            btreemap! {
                    "category" => category.to_string(),
                    "source" => crawler.get_source().to_string()
                },
        );

        let payload_json = serde_json::to_string(&payload).unwrap();

        let confirm = channel
            .basic_publish(
                "",
                &SETTINGS.amqp.queues.parse_category.name,
                BasicPublishOptions::default(),
                payload_json.into_bytes(),
                BasicProperties::default(),
            )
            .await?
            .await?;

        if confirm.is_nack() {
            let message = format!(
                "Message is not acknowledged! Queue: {}",
                SETTINGS.amqp.queues.parse_category.name
            );
            sentry::capture_message(message.as_str(), sentry::Level::Warning);
        } else {
            log::info!("Message acknowledged");
        }
    }

    Ok(())
}

fn add_producer_breadcrumb(message: &str, data: BTreeMap<&str, String>) {
    add_category_breadcrumb(
        message,
        data,
        ["producer.", &SETTINGS.amqp.queues.parse_category.name].join(""),
    );
}
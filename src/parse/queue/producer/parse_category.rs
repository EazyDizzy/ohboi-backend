use lapin::{Channel, Result};
use maplit::btreemap;
use serde::{Deserialize, Serialize};

use crate::parse::crawler::crawler::Crawler;
use crate::parse::crawler::mi_shop_com::MiShopComCrawler;
use crate::parse::crawler::samsung_shop_com_ua::SamsungShopComUaCrawler;
use crate::parse::db::entity::category::CategorySlug;
use crate::parse::db::entity::source::SourceName;
use crate::parse::queue::get_channel;
use crate::parse::queue::layer::produce::produce;
use crate::parse::queue::producer::add_producer_breadcrumb;
use crate::SETTINGS;

#[derive(Serialize, Deserialize)]
pub struct ParseCategoryMessage {
    pub category: CategorySlug,
    pub source: SourceName,
}

pub async fn start() -> Result<()> {
    let channel = get_channel().await?;
    // TODO get crawler based on enum
    produce_message_for_crawler(MiShopComCrawler {}, &channel).await?;
    produce_message_for_crawler(SamsungShopComUaCrawler {}, &channel).await?;

    Ok(())
}

async fn produce_message_for_crawler<T>(crawler: T, channel: &Channel) -> Result<()>
where
    T: Crawler,
{
    // TODO check if crawler is enabled
    for category in crawler.get_categories() {
        let payload = ParseCategoryMessage {
            category,
            source: crawler.get_source(),
        };
        add_producer_breadcrumb(
            "creating",
            btreemap! {
                "category" => category.to_string(),
                "source" => crawler.get_source().to_string()
            },
            &SETTINGS.queue_broker.queues.parse_category.name,
        );

        produce(&SETTINGS.queue_broker.queues.parse_category, &payload).await?;
    }

    Ok(())
}

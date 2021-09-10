use lapin::Result;
use serde::{Deserialize, Serialize};

use crate::db::entity::category::CategorySlug;
use crate::db::entity::source::SourceName;
use crate::parse::crawler::Crawler;
use crate::parse::crawler::MiShopComCrawler;
use crate::parse::crawler::SamsungShopComUaCrawler;
use crate::queue::layer::produce::produce;
use crate::SETTINGS;

#[derive(Serialize, Deserialize)]
pub struct ParseCategoryMessage {
    pub category: CategorySlug,
    pub source: SourceName,
}

pub async fn start() -> Result<()> {
    // TODO get crawler based on enum
    produce_message_for_crawler(MiShopComCrawler {}).await?;
    produce_message_for_crawler(SamsungShopComUaCrawler {}).await?;

    Ok(())
}

async fn produce_message_for_crawler<T>(crawler: T) -> Result<()>
where
    T: Crawler,
{
    // TODO check if crawler is enabled
    for category in crawler.get_categories() {
        let payload = ParseCategoryMessage {
            category,
            source: crawler.get_source(),
        };

        produce(&SETTINGS.queue_broker.queues.parse_category, &payload).await?;
    }

    Ok(())
}

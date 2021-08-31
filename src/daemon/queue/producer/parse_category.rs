use lapin::Result;
use maplit::btreemap;
use serde::{Deserialize, Serialize};

use crate::daemon::parse::crawler::crawler::Crawler;
use crate::daemon::parse::crawler::mi_shop_com::MiShopComCrawler;
use crate::daemon::parse::crawler::samsung_shop_com_ua::SamsungShopComUaCrawler;
use crate::daemon::db::entity::category::CategorySlug;
use crate::daemon::db::entity::source::SourceName;
use crate::daemon::queue::layer::produce::produce;
use crate::daemon::queue::producer::add_producer_breadcrumb;
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

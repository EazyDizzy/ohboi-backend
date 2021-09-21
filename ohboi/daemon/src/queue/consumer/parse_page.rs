use serde::{Deserialize, Serialize};

use lib::error_reporting;
use lib::error_reporting::ReportingContext;

use crate::db::entity::category::CategorySlug;
use crate::db::entity::source::SourceName;
use crate::parse::parse_category_page;
use crate::queue::layer::consume::consume;
use crate::{ConsumerName, SETTINGS};

#[derive(Serialize, Deserialize)]
pub struct ParsePageMessage {
    pub url: String,
    pub source: SourceName,
    pub category: CategorySlug,
}

pub async fn start() -> core::result::Result<(), ()> {
    consume(&SETTINGS.queue_broker.queues.parse_page, execute)
        .await
        .expect("Can't launch consumer");

    Ok(())
}

async fn execute(message: ParsePageMessage) -> Result<(), ()> {
    let parse_result = parse_category_page(&message.url, message.source, message.category).await;

    if parse_result.is_err() {
        let message = format!(
            "Page parsing failed! [{source}]({category}){error:?}",
            source = message.source,
            category = message.category,
            error = parse_result.err()
        );
        error_reporting::error(
            message.as_str(),
            &ReportingContext {
                executor: &ConsumerName::ParsePage,
                action: "execute",
            },
        );
        Err(())
    } else {
        Ok(())
    }
}

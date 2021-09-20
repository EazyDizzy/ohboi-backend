use lib::error_reporting;
use lib::error_reporting::ReportingContext;

use crate::parse::parse_category;
use crate::queue::layer::consume::consume;
use crate::queue::producer::parse_category::ParseCategoryMessage;
use crate::{ConsumerName, SETTINGS};

pub async fn start() -> core::result::Result<(), ()> {
    consume(&SETTINGS.queue_broker.queues.parse_category, execute)
        .await
        .expect("Can't launch consumer");

    Ok(())
}

async fn execute(message: ParseCategoryMessage) -> Result<(), ()> {
    let parse_result = parse_category(message.source, message.category).await;

    if parse_result.is_err() {
        let message = format!(
            "[{source}] Parsing failed! ({category}) {error:?}",
            error = parse_result.err(),
            source = message.source,
            category = message.category
        );
        error_reporting::error(
            message.as_str(),
            &ReportingContext {
                executor: &ConsumerName::ParseCategory,
                action: "execute",
            },
        );
        Err(())
    } else {
        Ok(())
    }
}

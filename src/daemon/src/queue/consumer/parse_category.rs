use crate::parse::parse_category;
use crate::queue::layer::consume::consume;
use crate::queue::producer::parse_category::ParseCategoryMessage;
use crate::SETTINGS;
use lib::local_sentry;

pub async fn start() -> core::result::Result<(), ()> {
    let _ = consume(&SETTINGS.queue_broker.queues.parse_category, execute)
        .await
        .expect("Can't launch consumer");

    Ok(())
}

async fn execute(message: ParseCategoryMessage) -> Result<(), ()> {
    let parse_result = parse_category(message.source, message.category).await;

    if parse_result.is_err() {
        let message = format!(
            "Parsing failed! [{source}]({category}) {error:?}",
            error = parse_result.err(),
            source = message.source,
            category = message.category
        );
        local_sentry::capture_message(message.as_str(), local_sentry::Level::Warning);
        Err(())
    } else {
        Ok(())
    }
}

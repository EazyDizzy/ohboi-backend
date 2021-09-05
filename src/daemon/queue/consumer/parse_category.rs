use crate::daemon::parse::parse_category;
use crate::daemon::queue::layer::consume::consume;
use crate::daemon::queue::producer::parse_category::ParseCategoryMessage;
use crate::SETTINGS;

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
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
        Err(())
    } else {
        Ok(())
    }
}

use maplit::btreemap;
use sentry::protocol::map::BTreeMap;
use serde::{Deserialize, Serialize};

use crate::daemon::db::entity::category::CategorySlug;
use crate::daemon::db::entity::source::SourceName;
use crate::daemon::parse::pub_api::parse_category_page::parse_category_page;
use crate::daemon::queue::layer::consume::consume;
use crate::local_sentry::add_category_breadcrumb;
use crate::SETTINGS;

#[derive(Serialize, Deserialize)]
pub struct ParsePageMessage {
    pub url: String,
    pub source: SourceName,
    pub category: CategorySlug,
}

pub async fn start() -> core::result::Result<(), ()> {
    let _ = consume(&SETTINGS.queue_broker.queues.parse_page, execute)
        .await
        .expect("Can't launch consumer");

    Ok(())
}

async fn execute(message: ParsePageMessage) -> Result<(), ()> {
    add_consumer_breadcrumb(
        "got message",
        btreemap! {
             "category" => message.category.to_string(),
             "source" => message.source.to_string(),
             "url" => message.url.to_string()
        },
    );

    let parse_result = parse_category_page(&message.url, message.source, message.category).await;

    if parse_result.is_err() {
        let message = format!(
            "Page parsing failed! [{source}]({category}){error:?}",
            source = message.source,
            category = message.category,
            error = parse_result.err()
        );
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
        Err(())
    } else {
        Ok(())
    }
}

fn add_consumer_breadcrumb(message: &str, data: BTreeMap<&str, String>) {
    add_category_breadcrumb(
        message,
        data,
        ["consumer.", &SETTINGS.queue_broker.queues.parse_page.name].join(""),
    );
}

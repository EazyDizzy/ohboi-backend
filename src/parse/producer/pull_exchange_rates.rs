use lapin::{BasicProperties, Result};
use lapin::options::BasicPublishOptions;
use maplit::*;
use sentry::protocol::map::BTreeMap;

use crate::local_sentry::add_category_breadcrumb;
use crate::parse::queue::get_channel;
use crate::SETTINGS;

pub async fn start() -> Result<()> {
    add_producer_breadcrumb(
        "creating",
        btreemap! {},
    );

    let channel = get_channel().await?;

    let confirm = channel
        .basic_publish(
            "",
            &SETTINGS.amqp.queues.pull_exchange_rates.name,
            BasicPublishOptions::default(),
            vec![],
            BasicProperties::default(),
        )
        .await?
        .await?;

    if confirm.is_nack() {
        let message = format!(
            "Message is not acknowledged! Queue: {}",
            SETTINGS.amqp.queues.pull_exchange_rates.name
        );
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
    } else {
        log::info!("Message acknowledged");
    }


    Ok(())
}

fn add_producer_breadcrumb(message: &str, data: BTreeMap<&str, String>) {
    add_category_breadcrumb(
        message,
        data,
        ["producer.", &SETTINGS.amqp.queues.pull_exchange_rates.name].join("").into(),
    );
}
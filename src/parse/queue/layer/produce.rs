use lapin::options::BasicPublishOptions;
use lapin::{BasicProperties, Result};
use serde::Serialize;

use crate::parse::queue::layer::get_channel;
use crate::parse::settings::QueueSettings;

pub async fn produce<T>(settings: &QueueSettings, message: &T) -> Result<()>
where
    T: ?Sized + Serialize,
{
    let channel = get_channel().await?;

    let payload_json = serde_json::to_string(message);
    let confirm = channel
        .basic_publish(
            "",
            &settings.name,
            BasicPublishOptions::default(),
            payload_json.unwrap().into_bytes(),
            BasicProperties::default(),
        )
        .await?
        .await?;

    if confirm.is_nack() {
        let message = format!(
            "Message is not acknowledged! Queue: {queue_name}",
            queue_name = &settings.name
        );
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
    } else {
        log::info!("Message acknowledged");
    }

    Ok(())
}
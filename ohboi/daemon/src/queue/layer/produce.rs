use lapin::options::BasicPublishOptions;
use lapin::{BasicProperties, Result};
use serde::Serialize;

use crate::queue::layer::get_channel;
use crate::settings::QueueSettings;
use lib::local_sentry;

pub async fn produce<Message>(settings: &QueueSettings, message: &Message) -> Result<()>
where
    Message: ?Sized + Serialize,
{
    let channel = get_channel().await?;

    let payload = serde_json::to_string(message).expect("Failed converting message to String");
    let confirm = channel
        .basic_publish(
            "",
            &settings.name,
            BasicPublishOptions::default(),
            payload.into_bytes(),
            BasicProperties::default(),
        )
        .await?
        .await?;

    if confirm.is_nack() {
        let message = format!(
            "Message is not acknowledged! Queue: {queue_name}",
            queue_name = &settings.name
        );
        local_sentry::capture_message(message.as_str(), local_sentry::Level::Warning);
    } else {
        log::info!("Message acknowledged");
    }

    Ok(())
}

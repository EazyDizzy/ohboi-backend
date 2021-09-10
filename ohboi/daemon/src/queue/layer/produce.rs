use lapin::options::BasicPublishOptions;
use lapin::{BasicProperties, Result};
use serde::Serialize;

use crate::queue::layer::get_channel;
use crate::settings::QueueSettings;
use lib::error_reporting;
use lib::error_reporting::ReportingContext;
use crate::queue::Executor;

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
        error_reporting::warning(message.as_str(), &ReportingContext {
            executor: &Executor::Queue,
            action: "produce"
        });
    }

    Ok(())
}

use lapin::options::QueueDeclareOptions;
use lapin::{types::FieldTable, Queue, Result};

use crate::queue::layer::get_channel;

pub async fn declare_queue(name: &str) -> Result<Queue> {
    let channel = get_channel().await?;

    let queue = channel
        .queue_declare(
            name,
            QueueDeclareOptions {
                passive: false,
                durable: true,
                exclusive: false,
                auto_delete: false,
                nowait: false,
            },
            FieldTable::default(),
        )
        .await?;
    Ok(queue)
}

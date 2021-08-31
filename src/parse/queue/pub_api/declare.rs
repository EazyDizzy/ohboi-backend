use lapin::options::QueueDeclareOptions;
use lapin::{types::FieldTable, Queue, Result};

use parse::settings::Settings;

use crate::parse::queue::get_channel;
use crate::SETTINGS;

pub async fn declare_all_queues() {
    let queues = [
        &SETTINGS.queue_broker.queues.parse_category.name,
        &SETTINGS.queue_broker.queues.parse_image.name,
        &SETTINGS.queue_broker.queues.parse_page.name,
        &SETTINGS.queue_broker.queues.pull_exchange_rates.name,
        &SETTINGS.queue_broker.queues.parse_details.name,
    ];

    for queue_name in &queues {
        let declare = declare_queue(queue_name).await;
        if declare.is_err() {
            log::error!("Queue declaration failed. {} {:?}", queue_name, declare);
        }
    }
}

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

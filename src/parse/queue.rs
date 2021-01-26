use lapin::{Channel, Connection, ConnectionProperties, Queue, Result, types::FieldTable};
use lapin::options::QueueDeclareOptions;

use crate::SETTINGS;

pub async fn declare_image_upload_queue() -> Result<Queue> {
    let channel = get_channel().await?;

    let queue = channel
        .queue_declare(
            &SETTINGS.amqp.queues.image_upload.name,
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

pub async fn declare_crawler_category_queue() -> Result<Queue> {
    let channel = get_channel().await?;

    let queue = channel
        .queue_declare(
            &SETTINGS.amqp.queues.crawler_category.name,
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

pub async fn get_channel() -> Result<Channel> {
    let address = &SETTINGS.amqp.url;

    let conn = Connection::connect(
        &address,
        ConnectionProperties::default(),
    )
        .await?;

    let channel = conn.create_channel().await?;

    Ok(channel)
}
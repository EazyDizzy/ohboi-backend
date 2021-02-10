use lapin::{BasicProperties, Channel, Connection, ConnectionProperties, Queue, Result, types::FieldTable};
use lapin::options::{BasicPublishOptions, QueueDeclareOptions};
use maplit::*;

use crate::parse::service::cloud_uploader::add_uploader_breadcrumb;
use crate::parse::consumer::parse_image::UploadImageMessage;
use crate::parse::consumer::parse_page::ParsePageMessage;
use crate::SETTINGS;

pub async fn declare_parse_page_queue() -> Result<Queue> {
    let channel = get_channel().await?;

    let queue = channel
        .queue_declare(
            &SETTINGS.amqp.queues.parse_page.name,
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

pub async fn declare_parse_image_queue() -> Result<Queue> {
    let channel = get_channel().await?;

    let queue = channel
        .queue_declare(
            &SETTINGS.amqp.queues.parse_image.name,
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

pub async fn declare_parse_category_queue() -> Result<Queue> {
    let channel = get_channel().await?;

    let queue = channel
        .queue_declare(
            &SETTINGS.amqp.queues.parse_category.name,
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

pub async fn postpone_page_parsing(message: ParsePageMessage) -> Result<()> {
    let breadcrumb_data = btreemap! {
                    "category" => message.category.clone().to_string(),
                    "source" => message.source.clone().to_string(),
                    "url" => message.url.to_string()
                };
    add_uploader_breadcrumb("scheduling later upload", breadcrumb_data);

    let channel = get_channel().await?;

    let payload_json = serde_json::to_string(&message);
    let confirm = channel
        .basic_publish(
            "",
            &SETTINGS.amqp.queues.parse_page.name,
            BasicPublishOptions::default(),
            payload_json.unwrap().into_bytes(),
            BasicProperties::default(),
        )
        .await?
        .await?;

    if confirm.is_nack() {
        let message = format!(
            "Message is not acknowledged! Queue: {}",
            SETTINGS.amqp.queues.parse_page.name
        );
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
    } else {
        log::info!("Message acknowledged");
    }

    Ok(())
}

pub async fn postpone_image_parsing(message: UploadImageMessage) -> Result<()> {
    let breadcrumb_data = btreemap! {
                    "file_path" => message.file_path.clone(),
                    "image_url" => message.image_url.clone(),
                    "external_id" => message.external_id.to_string()
                };
    add_uploader_breadcrumb("scheduling later upload", breadcrumb_data);

    let channel = get_channel().await?;

    let payload_json = serde_json::to_string(&message);
    let confirm = channel
        .basic_publish(
            "",
            &SETTINGS.amqp.queues.parse_image.name,
            BasicPublishOptions::default(),
            payload_json.unwrap().into_bytes(),
            BasicProperties::default(),
        )
        .await?
        .await?;

    if confirm.is_nack() {
        let message = format!(
            "Message is not acknowledged! Queue: {}",
            SETTINGS.amqp.queues.parse_image.name
        );
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
    } else {
        log::info!("Message acknowledged");
    }

    Ok(())
}
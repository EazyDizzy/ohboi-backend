use futures::{Future, StreamExt};
use lapin::message::Delivery;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions, BasicQosOptions},
    types::FieldTable,
    Consumer, Result,
};
use serde::de;

use crate::queue::layer::get_channel;
use crate::settings::QueueSettings;

// TODO parallel with a help of set_delegate
pub async fn consume<F, Fut, Message>(settings: &QueueSettings, consumer_callback: F) -> Result<()>
where
    F: Fn(Message) -> Fut,
    Fut: Future<Output = core::result::Result<(), ()>>,
    Message: de::DeserializeOwned,
{
    let mut consumer = get_consumer(settings).await;

    while let Some(delivery) = consumer.next().await {
        let (_, delivery) =
            delivery.expect(&format!("[{}] Can't consume queue message.", settings.name));

        let message = parse_message(&delivery, settings);
        let job_result = consumer_callback(message).await;

        match job_result {
            Ok(_) => mark_success(&delivery).await,
            Err(_) => requeue(&delivery).await,
        };
    }

    Ok(())
}

fn parse_message<Message>(delivery: &Delivery, settings: &QueueSettings) -> Message
where
    Message: de::DeserializeOwned,
{
    let message = std::str::from_utf8(&delivery.data).expect(&format!(
        "[{}] Message is not a valid ut8 string.",
        settings.name
    ));

    serde_json::from_str(&message).expect("Failed to parse message")
}

async fn get_consumer(settings: &QueueSettings) -> Consumer {
    let channel = get_channel().await.expect("Failed to get channel");

    channel
        .basic_qos(settings.prefetch, BasicQosOptions { global: true })
        .await
        .expect("Failed to set basic qos");

    channel
        .basic_consume(
            &settings.name,
            [&settings.name, "_consumer"].join("").as_str(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to get consumer")
}

async fn mark_success(delivery: &Delivery) {
    delivery
        .ack(BasicAckOptions { multiple: false })
        .await
        .expect("acknowledgment failed");
}

async fn requeue(delivery: &Delivery) {
    // TODO requeue with delay https://blog.rabbitmq.com/posts/2015/04/scheduling-messages-with-rabbitmq

    delivery
        .nack(BasicNackOptions {
            requeue: true,
            multiple: false,
        })
        .await
        .expect("not-acknowledgment failed");
}

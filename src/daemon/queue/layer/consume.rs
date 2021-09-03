use futures::{Future, StreamExt};
use lapin::message::Delivery;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions, BasicQosOptions},
    types::FieldTable,
    Consumer, Result,
};
use maplit::btreemap;
use sentry::protocol::map::BTreeMap;

use crate::daemon::queue::layer::get_channel;
use crate::daemon::settings::QueueSettings;
use crate::local_sentry::add_category_breadcrumb;

pub async fn consume<F, Fut>(settings: &QueueSettings, consumer_callback: F) -> Result<()>
where
    F: Fn(String) -> Fut,
    Fut: Future<Output = core::result::Result<(), ()>>,
{
    let mut consumer = get_consumer(settings).await;

    while let Some(delivery) = consumer.next().await {
        let (_, delivery) =
            delivery.expect(&format!("[{}] Can't consume queue message.", settings.name));

        let message = parse_message(&delivery, settings);
        // Todo pass &str or parsed_message
        let job_result = consumer_callback(message.to_string()).await;

        match job_result {
            Ok(_) => job_success(&delivery).await,
            Err(_) => job_failed(&delivery).await,
        };
    }

    Ok(())
}

fn parse_message<'delivery>(
    delivery: &'delivery Delivery,
    settings: &QueueSettings,
) -> &'delivery str {
    add_consumer_breadcrumb("got message", btreemap! {}, &settings.name);

    std::str::from_utf8(&delivery.data).expect(&format!(
        "[{}] Message is not a valid ut8 string.",
        settings.name
    ))
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

async fn job_success(delivery: &Delivery) {
    delivery
        .ack(BasicAckOptions { multiple: false })
        .await
        .expect("acknowledgment failed");
}

async fn job_failed(delivery: &Delivery) {
    // TODO requeue with delay https://blog.rabbitmq.com/posts/2015/04/scheduling-messages-with-rabbitmq

    delivery
        .nack(BasicNackOptions {
            requeue: true,
            multiple: false,
        })
        .await
        .expect("not-acknowledgment failed");
}

fn add_consumer_breadcrumb(message: &str, data: BTreeMap<&str, String>, consumer_name: &str) {
    add_category_breadcrumb(message, data, ["consumer.", consumer_name].join(""));
}

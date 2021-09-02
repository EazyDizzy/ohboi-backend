use futures::StreamExt;
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

type ConsumerCallBack = fn(String) -> core::result::Result<(), ()>;

pub async fn consume(settings: &QueueSettings, consumer_callback: ConsumerCallBack) -> Result<()> {
    let mut consumer = get_consumer(settings).await;

    while let Some(delivery) = consumer.next().await {
        let (_, delivery) =
            delivery.expect(&format!("[{}] Can't consume queue message.", settings.name));

        add_consumer_breadcrumb("got message", btreemap! {}, &settings.name);

        let message = std::str::from_utf8(&delivery.data).expect(&format!(
            "[{}] Message is not a valid ut8 string.",
            settings.name
        ));
        // Todo pass &str or parsed_message
        let job_result = consumer_callback(message.to_string());

        if job_result.is_ok() {
            job_success(delivery).await;
        } else {
            job_failed(delivery).await;
        }
    }

    Ok(())
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

async fn job_success(delivery: Delivery) {
    delivery
        .ack(BasicAckOptions { multiple: false })
        .await
        .expect("acknowledgment failed");
}

async fn job_failed(delivery: Delivery) {
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

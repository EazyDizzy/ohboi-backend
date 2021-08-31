use futures::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions, BasicQosOptions},
    types::FieldTable,
    Result,
};
use maplit::btreemap;
use sentry::protocol::map::BTreeMap;

use crate::local_sentry::add_category_breadcrumb;
use crate::parse::settings::QueueSettings;
use crate::parse::queue::layer::get_channel;

type ConsumerCallBack = fn(String) -> core::result::Result<(), ()>;

pub async fn consume(
    settings: &QueueSettings,
    consumer_callback: ConsumerCallBack,
) -> Result<()> {
    let channel = get_channel().await?;
    channel
        .basic_qos(settings.prefetch, BasicQosOptions { global: true })
        .await?;

    let mut consumer = channel
        .basic_consume(
            &settings.name,
            [&settings.name, "_consumer"].join("").as_str(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(delivery) = consumer.next().await {
        let (_, delivery) =
            delivery.expect(&format!("[{}] Can't consume queue message.", settings.name));

        add_consumer_breadcrumb("got message", btreemap! {}, &settings.name);

        let message = std::str::from_utf8(&delivery.data).expect(&format!(
            "[{}] Message is not a valid ut8 string.",
            settings.name
        ));
        // Todo pass &str
        let job_result = consumer_callback(message.to_string());

        if job_result.is_ok() {
            delivery
                .ack(BasicAckOptions { multiple: false })
                .await
                .expect("ack");
        } else {
            // TODO requeue with delay https://blog.rabbitmq.com/posts/2015/04/scheduling-messages-with-rabbitmq
            delivery
                .nack(BasicNackOptions {
                    requeue: true,
                    multiple: false,
                })
                .await
                .expect("nack");
        }
    }

    Ok(())
}

fn add_consumer_breadcrumb(message: &str, data: BTreeMap<&str, String>, consumer_name: &str) {
    add_category_breadcrumb(message, data, ["consumer.", consumer_name].join(""));
}

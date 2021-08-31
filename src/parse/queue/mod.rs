use lapin::options::{BasicPublishOptions, QueueDeclareOptions};
use lapin::{
    types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties, Queue, Result,
};
use maplit::btreemap;
use sentry::types::protocol::latest::map::BTreeMap;

use consumer::parse_details::ParseDetailsMessage;
use consumer::parse_image::UploadImageMessage;
use consumer::parse_page::ParsePageMessage;
pub use producer::postpone;

use crate::local_sentry::add_category_breadcrumb;
use crate::parse::queue::layer::produce::produce;
use crate::{ConsumerName, ProducerName, SETTINGS};

mod consumer;
mod layer;
mod producer;

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

pub async fn start_producer(name: ProducerName) -> Result<()> {
    match name {
        ProducerName::ParseCategory => producer::parse_category::start().await,
        ProducerName::PullExchangeRates => producer::pull_exchange_rates::start().await,
    }
}

pub async fn start_consumer(name: ConsumerName) -> core::result::Result<(), ()> {
    match name {
        ConsumerName::ParseCategory => consumer::parse_category::start().await,
        ConsumerName::ParseImage => consumer::parse_image::start().await,
        ConsumerName::ParsePage => consumer::parse_page::start().await,
        ConsumerName::PullExchangeRates => consumer::pull_exchange_rates::start().await,
        ConsumerName::ParseDetails => consumer::parse_details::start().await,
    }
}

async fn get_channel() -> Result<Channel> {
    let address = &SETTINGS.queue_broker.url;
    let conn = Connection::connect(&address, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    Ok(channel)
}

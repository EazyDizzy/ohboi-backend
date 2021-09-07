use lapin::Result;

use crate::queue::{consumer, producer};
use crate::{ConsumerName, ProducerName};

pub async fn launch_producer(name: ProducerName) -> Result<()> {
    match name {
        ProducerName::ParseCategory => producer::parse_category::start().await,
        ProducerName::PullExchangeRates => producer::pull_exchange_rates::start().await,
    }
}

pub async fn launch_consumer(name: ConsumerName) -> core::result::Result<(), ()> {
    match name {
        ConsumerName::ParseCategory => consumer::parse_category::start().await,
        ConsumerName::ParseImage => consumer::parse_image::start().await,
        ConsumerName::ParsePage => consumer::parse_page::start().await,
        ConsumerName::PullExchangeRates => consumer::pull_exchange_rates::start().await,
        ConsumerName::ParseDetails => consumer::parse_details::start().await,
    }
}

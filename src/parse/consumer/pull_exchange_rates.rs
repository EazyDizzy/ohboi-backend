use futures::StreamExt;
use lapin::{options::*, Result, types::FieldTable};
use maplit::*;
use sentry::protocol::map::BTreeMap;
use serde::Deserialize;
use serde_json::error::Result as SerdeResult;

use crate::parse::db::repository::exchange_rate::create_or_update;
use crate::local_sentry::add_category_breadcrumb;
use crate::my_enum::CurrencyEnum;
use crate::parse::queue::get_channel;
use crate::parse::requester::get_data;
use crate::SETTINGS;

#[derive(Deserialize, Debug)]
struct ExchangeApiResponse {
    success: bool,
    rates: ExchangeApiRates,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
struct ExchangeApiRates {
    rub: f32,
    uah: f32,
    usd: f32,
}

pub async fn start() -> Result<()> {
    let channel = get_channel().await?;
    channel.basic_qos(
        SETTINGS.amqp.queues.pull_exchange_rates.prefetch,
        BasicQosOptions { global: true },
    ).await?;

    let mut consumer = channel
        .basic_consume(
            &SETTINGS.amqp.queues.pull_exchange_rates.name,
            [&SETTINGS.amqp.queues.pull_exchange_rates.name, "_consumer"].join("").as_str(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(delivery) = consumer.next().await {
        let (_, delivery) = delivery.expect("error in consumer");

        add_consumer_breadcrumb(
            "got message",
            btreemap! {},
        );

        let response = get_data("https://api.exchangerate.host/latest?base=EUR&symbols=UAH,USD,RUB").await;

        if response.is_err() {
            let message = format!("Request for exchange rates failed!  {error:?}", error = response.err());
            sentry::capture_message(message.as_str(), sentry::Level::Warning);
            delivery.nack(BasicNackOptions { requeue: true, multiple: false }).await.expect("nack");
            continue;
        }

        let api_response: SerdeResult<ExchangeApiResponse> = serde_json::from_str(response.unwrap().as_str());

        if api_response.is_err() {
            let message = format!("Parsing of response failed!  {error:?}", error = api_response.err());
            sentry::capture_message(message.as_str(), sentry::Level::Warning);
            delivery.nack(BasicNackOptions { requeue: true, multiple: false }).await.expect("nack");
            continue;
        }

        let response = api_response.unwrap();

        if !response.success {
            sentry::capture_message("Response from api is not success!", sentry::Level::Warning);
            delivery.nack(BasicNackOptions { requeue: true, multiple: false }).await.expect("nack");
            continue;
        }

        let save_result = create_or_update(&CurrencyEnum::RUB, response.rates.rub)
            && create_or_update(&CurrencyEnum::UAH, response.rates.uah)
            && create_or_update(&CurrencyEnum::USD, response.rates.usd)
            && create_or_update(&CurrencyEnum::EUR, 1.0)
            ;

        if !save_result {
            sentry::capture_message("Saving of exchange rate failed!", sentry::Level::Warning);
            delivery.nack(BasicNackOptions { requeue: true, multiple: false }).await.expect("nack");
        } else {
            delivery.ack(BasicAckOptions { multiple: false }).await.expect("ack");
        }
    }

    Ok(())
}

fn add_consumer_breadcrumb(message: &str, data: BTreeMap<&str, String>) {
    add_category_breadcrumb(
        message,
        data,
        ["consumer.", &SETTINGS.amqp.queues.pull_exchange_rates.name].join("").into(),
    );
}
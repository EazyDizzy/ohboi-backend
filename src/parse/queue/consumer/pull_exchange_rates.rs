use crossbeam::channel;
use sentry::protocol::map::BTreeMap;
use serde::Deserialize;
use serde_json::error::Result as SerdeResult;
use tokio::runtime::Handle;

use crate::local_sentry::add_category_breadcrumb;
use crate::my_enum::CurrencyEnum;
use crate::parse::queue::layer::consume::consume;
use crate::parse::db::repository::exchange_rate::create_or_update;
use crate::parse::service::requester::get_data;
use crate::SETTINGS;

#[derive(Deserialize)]
struct ExchangeApiResponse {
    success: bool,
    rates: ExchangeApiRates,
}

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct ExchangeApiRates {
    rub: f32,
    uah: f32,
    usd: f32,
}

pub async fn start() -> Result<(), ()> {
    let _ = consume(&SETTINGS.queue_broker.queues.pull_exchange_rates, |message| {
        let (snd, rcv) = channel::bounded(1);

        let _ = Handle::current().spawn(async move {
            let rs = execute().await;
            let _ = snd.send(rs);
        });

        rcv.recv().unwrap()
    })
    .await;

    Ok(())
}

async fn execute() -> Result<(), ()> {
    let response =
        get_data("https://api.exchangerate.host/latest?base=EUR&symbols=UAH,USD,RUB").await;

    if response.is_err() {
        let message = format!(
            "Request for exchange rates failed!  {error:?}",
            error = response.err()
        );
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
        return Err(());
    }

    let api_response: SerdeResult<ExchangeApiResponse> =
        serde_json::from_str(response.unwrap().as_str());

    if api_response.is_err() {
        let message = format!(
            "Parsing of response failed!  {error:?}",
            error = api_response.err()
        );
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
        return Err(());
    }

    let response = api_response.unwrap();

    if !response.success {
        sentry::capture_message("Response from api is not success!", sentry::Level::Warning);
        return Err(());
    }

    let save_result = create_or_update(CurrencyEnum::RUB, response.rates.rub)
        && create_or_update(CurrencyEnum::UAH, response.rates.uah)
        && create_or_update(CurrencyEnum::USD, response.rates.usd)
        && create_or_update(CurrencyEnum::EUR, 1.0);

    if save_result {
        Ok(())
    } else {
        sentry::capture_message("Saving of exchange rate failed!", sentry::Level::Warning);
        Err(())
    }
}

fn add_consumer_breadcrumb(message: &str, data: BTreeMap<&str, String>) {
    add_category_breadcrumb(
        message,
        data,
        ["consumer.", &SETTINGS.queue_broker.queues.pull_exchange_rates.name].join(""),
    );
}

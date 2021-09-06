use serde::Deserialize;
use serde_json::error::Result as SerdeResult;

use crate::db::repository::exchange_rate::create_or_update;
use crate::queue::layer::consume::consume;
use crate::service::request::get;
use lib::my_enum::CurrencyEnum;
use crate::SETTINGS;
use lib::local_sentry;

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
    let _ = consume(&SETTINGS.queue_broker.queues.pull_exchange_rates, execute)
        .await
        .expect("Can't launch consumer");

    Ok(())
}

async fn execute(_message: String) -> Result<(), ()> {
    let response =
        get("https://api.exchangerate.host/latest?base=EUR&symbols=UAH,USD,RUB").await;

    if response.is_err() {
        let message = format!(
            "Request for exchange rates failed!  {error:?}",
            error = response.err()
        );
        local_sentry::capture_message(message.as_str(), local_sentry::Level::Warning);
        return Err(());
    }

    let api_response: SerdeResult<ExchangeApiResponse> = serde_json::from_str(
        &response
            .expect("Failed to daemon ExchangeApiResponse. Maybe response format has changed?"),
    );

    if api_response.is_err() {
        let message = format!(
            "Parsing of response failed!  {error:?}",
            error = api_response.err()
        );
        local_sentry::capture_message(message.as_str(), local_sentry::Level::Warning);
        return Err(());
    }

    // We have already checked for error
    let response = api_response.expect("");

    if !response.success {
        local_sentry::capture_message("Response from api is not success!", local_sentry::Level::Warning);
        return Err(());
    }

    let save_result = create_or_update(CurrencyEnum::RUB, response.rates.rub)
        && create_or_update(CurrencyEnum::UAH, response.rates.uah)
        && create_or_update(CurrencyEnum::USD, response.rates.usd)
        && create_or_update(CurrencyEnum::EUR, 1.0);

    if save_result {
        Ok(())
    } else {
        local_sentry::capture_message("Saving of exchange rate failed!", local_sentry::Level::Warning);
        Err(())
    }
}

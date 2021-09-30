use bigdecimal::ToPrimitive;
use cached::proc_macro::cached;
use chrono::Utc;
use diesel::{QueryDsl, RunQueryDsl};
use bigdecimal::BigDecimal;

use crate::db;
use crate::db::exchange_rate::entity::{ExchangeRate, NewExchangeRate};
use crate::diesel::prelude::*;
use crate::my_enum::CurrencyEnum;
use crate::schema::exchange_rate;

#[cached(size = 4, time = 600)]
pub fn try_get_exchange_rate_by_code(sought_currency: CurrencyEnum) -> f64 {
    get_exchange_rate_by_code(sought_currency)
        .expect("No exchange rate found in db")
        .rate
        .to_f64()
        .expect("Rate is not f64")
}

pub fn create_or_update(currency: CurrencyEnum, rate: f32) -> bool {
    let existed_rate = get_exchange_rate_by_code(currency);

    if existed_rate.is_none() {
        create(currency, rate)
    } else {
        update(currency, rate)
    }
}

fn get_exchange_rate_by_code(sought_currency: CurrencyEnum) -> Option<ExchangeRate> {
    use crate::schema::exchange_rate::dsl::{currency, exchange_rate};

    let connection = &db::establish_connection();

    let target = exchange_rate.filter(currency.eq(sought_currency));
    let results: Vec<ExchangeRate> = target
        .limit(1)
        .load::<ExchangeRate>(connection)
        .expect("Error loading exchange rate");

    results.into_iter().next()
}

fn create(currency: CurrencyEnum, rate: f32) -> bool {
    let connection = &db::establish_connection();
    let now = Utc::now();

    let new_rate = NewExchangeRate {
        currency,
        rate: BigDecimal::from(rate),
        updated_at: &now.naive_utc(),
    };

    let insert_result = diesel::insert_into(exchange_rate::table)
        .values(&new_rate)
        .get_result::<ExchangeRate>(connection);

    insert_result.is_ok()
}

fn update(sought_currency: CurrencyEnum, new_rate: f32) -> bool {
    use crate::schema::exchange_rate::dsl::{currency, exchange_rate, rate, updated_at};

    let connection = &db::establish_connection();
    let now = Utc::now();

    let target = exchange_rate.filter(currency.eq(sought_currency));

    let update_result = diesel::update(target)
        .set((
            rate.eq(BigDecimal::from(new_rate)),
            updated_at.eq(&now.naive_utc()),
        ))
        .execute(connection)
        .expect("Failed to update rate");

    update_result == 1
}

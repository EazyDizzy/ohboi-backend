use bigdecimal::BigDecimal;
use chrono::Utc;
use diesel::{QueryDsl, RunQueryDsl};

use crate::diesel::prelude::*;
use crate::parse::db;
use crate::parse::db::entity::{ExchangeRate, NewExchangeRate};
use crate::schema::exchange_rate;

pub fn get_exchange_rate_by_code(sought_currency_code: &str) -> Option<ExchangeRate> {
    use crate::schema::exchange_rate::dsl::*;

    let connection = &db::establish_connection();

    let target = exchange_rate.filter(currency_code.eq(sought_currency_code));
    let results: Vec<ExchangeRate> = target
        .limit(1)
        .load::<ExchangeRate>(connection)
        .expect("Error loading exchange rate");

    results.into_iter().next()
}

pub fn create_or_update(currency_code: &str, rate: f32) -> bool {
    let existed_rate = get_exchange_rate_by_code(currency_code);

    if existed_rate.is_none() {
        create(
            currency_code,
            rate,
        )
    } else {
        update(
            currency_code,
            rate,
        )
    }
}

fn create(currency_code: &str, rate: f32) -> bool {
    let connection = &db::establish_connection();
    let now = Utc::now();

    let new_rate = NewExchangeRate {
        currency_code,
        rate: BigDecimal::from(rate),
        updated_at: &now.naive_utc(),
    };

    let insert_result = diesel::insert_into(exchange_rate::table)
        .values(&new_rate)
        .get_result::<ExchangeRate>(connection);

    insert_result.is_ok()
}

fn update(sought_currency_code: &str, new_rate: f32) -> bool {
    use crate::schema::exchange_rate::dsl::*;

    let connection = &db::establish_connection();
    let now = Utc::now();

    let target = exchange_rate.filter(currency_code.eq(sought_currency_code));

    let update_result = diesel::update(target)
        .set((
            rate.eq(BigDecimal::from(new_rate)),
            updated_at.eq(&now.naive_utc())
        ))
        .execute(connection)
        .expect("Failed to update rate");

    update_result == 1
}
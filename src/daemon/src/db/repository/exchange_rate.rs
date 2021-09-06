use bigdecimal::BigDecimal;
use chrono::Utc;
use lib::diesel::{QueryDsl, RunQueryDsl};

use lib::db;
use lib::db::entity::exchange_rate::{ExchangeRate, NewExchangeRate};
use lib::db::repository::exchange_rate::get_exchange_rate_by_code;
use lib::diesel::prelude::*;
use lib::my_enum::CurrencyEnum;
use lib::schema::exchange_rate;

pub fn create_or_update(currency: CurrencyEnum, rate: f32) -> bool {
    let existed_rate = get_exchange_rate_by_code(currency);

    if existed_rate.is_none() {
        create(
            currency,
            rate,
        )
    } else {
        update(
            currency,
            rate,
        )
    }
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
    use lib::schema::exchange_rate::dsl::{currency, exchange_rate, rate, updated_at};

    let connection = &db::establish_connection();
    let now = Utc::now();

    let target = exchange_rate.filter(currency.eq(sought_currency));

    let update_result = diesel::update(target)
        .set((
            rate.eq(BigDecimal::from(new_rate)),
            updated_at.eq(&now.naive_utc())
        ))
        .execute(connection)
        .expect("Failed to update rate");

    update_result == 1
}
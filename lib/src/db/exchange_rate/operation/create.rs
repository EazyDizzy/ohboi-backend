use bigdecimal::BigDecimal;
use diesel::{QueryDsl, RunQueryDsl};

use crate::db;
use crate::db::exchange_rate::entity::{ExchangeRate, NewExchangeRate};
use crate::db::exchange_rate::operation::find_exchange_rate_by_code;
use crate::diesel::prelude::*;
use crate::my_enum::CurrencyEnum;
use crate::schema::exchange_rate;

pub fn upsert(currency: CurrencyEnum, rate: f32) -> bool {
    let existed_rate = find_exchange_rate_by_code(currency);

    if existed_rate.is_none() {
        create(currency, rate)
    } else {
        update(currency, rate)
    }
}

fn create(currency: CurrencyEnum, rate: f32) -> bool {
    let new_rate = NewExchangeRate {
        currency,
        rate: BigDecimal::from(rate),
    };

    let insert_result = diesel::insert_into(exchange_rate::table)
        .values(&new_rate)
        .on_conflict((
            exchange_rate::currency
        ))
        .do_update()
        .set((
            exchange_rate::rate.eq(&new_rate.rate),
        ))
        .execute(&db::establish_connection());

    insert_result.is_ok()
}

fn update(sought_currency: CurrencyEnum, new_rate: f32) -> bool {
    use crate::schema::exchange_rate::dsl::{currency, exchange_rate, rate};

    let target = exchange_rate.filter(currency.eq(sought_currency));

    let update_result = diesel::update(target)
        .set((rate.eq(BigDecimal::from(new_rate)),))
        .execute(&db::establish_connection())
        .expect("Failed to update rate");

    update_result == 1
}
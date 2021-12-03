use bigdecimal::ToPrimitive;
use cached::proc_macro::cached;
use diesel::{QueryDsl, RunQueryDsl};

use crate::db;
use crate::db::exchange_rate::entity::ExchangeRate;
use crate::diesel::prelude::*;
use crate::my_enum::CurrencyEnum;

#[cached(size = 4, time = 600)]
pub fn get_exchange_rate_by_code(sought_currency: CurrencyEnum) -> f64 {
    find_exchange_rate_by_code(sought_currency)
        .expect("No exchange rate found in db")
        .rate
        .to_f64()
        .expect("Rate is not f64")
}

pub(super) fn find_exchange_rate_by_code(sought_currency: CurrencyEnum) -> Option<ExchangeRate> {
    use crate::schema::exchange_rate::dsl::{currency, exchange_rate};

    let connection = &db::establish_connection();

    let target = exchange_rate.filter(currency.eq(sought_currency));
    let results: Vec<ExchangeRate> = target
        .limit(1)
        .load::<ExchangeRate>(connection)
        .expect("Error loading exchange rate");

    results.into_iter().next()
}

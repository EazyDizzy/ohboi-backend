use diesel::{QueryDsl, RunQueryDsl};

use crate::common::db;
use crate::common::db::entity::exchange_rate::ExchangeRate;
use crate::diesel::prelude::*;
use crate::my_enum::CurrencyEnum;

pub fn get_exchange_rate_by_code(sought_currency: CurrencyEnum) -> Option<ExchangeRate> {
    use crate::schema::exchange_rate::dsl::{currency, exchange_rate};

    let connection = &db::establish_connection();

    let target = exchange_rate.filter(currency.eq(sought_currency));
    let results: Vec<ExchangeRate> = target
        .limit(1)
        .load::<ExchangeRate>(connection)
        .expect("Error loading exchange rate");

    results.into_iter().next()
}
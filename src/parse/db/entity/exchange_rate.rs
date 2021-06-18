use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::schema::exchange_rate;

#[derive(Serialize, Queryable, Debug)]
pub struct ExchangeRate {
    pub id: i32,
    pub currency_code: String,
    pub rate: BigDecimal,

    #[serde(skip)]
    pub updated_at: NaiveDateTime,
}


#[derive(Insertable)]
#[table_name = "exchange_rate"]
pub struct NewExchangeRate<'a> {
    pub currency_code: &'a str,
    pub rate: BigDecimal,

    pub updated_at: &'a NaiveDateTime,
}
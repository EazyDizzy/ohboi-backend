use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::my_enum::CurrencyEnum;
use crate::schema::exchange_rate;

#[derive(Serialize, Queryable, Debug)]
pub struct ExchangeRate {
    pub id: i32,
    pub currency: CurrencyEnum,
    pub rate: BigDecimal,

    #[serde(skip)]
    pub updated_at: NaiveDateTime,
}


#[derive(Insertable)]
#[table_name = "exchange_rate"]
pub struct NewExchangeRate<'a> {
    pub currency: &'a CurrencyEnum,
    pub rate: BigDecimal,

    pub updated_at: &'a NaiveDateTime,
}
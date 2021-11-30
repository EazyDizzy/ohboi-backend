use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

use crate::my_enum::CurrencyEnum;
use crate::schema::exchange_rate;

#[derive(Queryable, Debug)]
pub struct ExchangeRate {
    pub id: i32,
    pub currency: CurrencyEnum,
    pub rate: BigDecimal,

    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "exchange_rate"]
pub struct NewExchangeRate {
    pub currency: CurrencyEnum,
    pub rate: BigDecimal,
}

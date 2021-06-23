use bigdecimal::ToPrimitive;

use crate::common::db::repository::exchange_rate::get_exchange_rate_by_code;
use crate::my_enum::CurrencyEnum;

#[allow(dead_code)]
pub fn convert_from_with_rate(price: f64, rate: f64) -> f64 {
    price / rate
}

#[allow(dead_code)]
pub fn convert_to_with_rate(price: f64, rate: f64) -> f64 {
    price * rate
}
use bigdecimal::ToPrimitive;

use crate::my_enum::CurrencyEnum;
use crate::parse::db::repository::exchange_rate::get_exchange_rate_by_code;

pub fn convert_from_with_rate(price: f64, rate: f64) -> f64 {
    price / rate
}

pub fn convert_to(price: f64, to: &CurrencyEnum) -> f64 {
    let rate = get_exchange_rate_by_code(to).unwrap();

    price * rate.rate.to_f64().unwrap()
}
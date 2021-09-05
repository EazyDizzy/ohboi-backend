use crate::db::repository::exchange_rate::try_get_exchange_rate_by_code;
use crate::my_enum::CurrencyEnum;

#[allow(dead_code)]
pub fn convert_from(price: f64, currency: CurrencyEnum) -> f64 {
    let rate = try_get_exchange_rate_by_code(currency);

    convert_from_with_rate(price, rate)
}

#[allow(dead_code)]
pub fn convert_from_with_rate(price: f64, rate: f64) -> f64 {
    price / rate
}

#[allow(dead_code)]
pub fn convert_to_with_rate(price: f64, rate: f64) -> f64 {
    price * rate
}

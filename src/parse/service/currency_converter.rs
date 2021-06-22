use bigdecimal::ToPrimitive;

use crate::parse::db::repository::exchange_rate::get_exchange_rate_by_code;

pub fn convert_from(price: f64, from: &str) -> f64 {
    let rate = get_exchange_rate_by_code(from).unwrap();

    price / rate.rate.to_f64().unwrap()
}

pub fn convert_to(price: f64, to: &str) -> f64 {
    let rate = get_exchange_rate_by_code(to).unwrap();

    price * rate.rate.to_f64().unwrap()
}
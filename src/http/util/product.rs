use crate::http::db::product::entity::Product;
use bigdecimal::{BigDecimal, ToPrimitive};
use lib::service::currency_converter::convert_to_with_rate;

pub fn convert_product_prices(product: &mut Product, rate: f64) {
    product.highest_price = BigDecimal::from(convert_to_with_rate(
        product.highest_price.to_f64().unwrap(),
        rate,
    ));
    product.lowest_price = BigDecimal::from(convert_to_with_rate(
        product.lowest_price.to_f64().unwrap(),
        rate,
    ));
}

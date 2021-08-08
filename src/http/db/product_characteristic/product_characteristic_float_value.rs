use bigdecimal::BigDecimal;
use serde::Serialize;

use crate::schema::product_characteristic_float_value;

#[derive(Serialize, Queryable)]
pub struct ProductCharacteristicFloatValue {
    pub id: i32,
    pub value: BigDecimal,
}

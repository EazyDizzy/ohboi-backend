use bigdecimal::BigDecimal;
use serde::Serialize;

use lib::schema::product_characteristic_float_value;

#[derive(Serialize, Queryable)]
pub struct ProductCharacteristicFloatValue {
    pub id: i32,
    pub value: BigDecimal,
}

#[derive(Insertable)]
#[table_name = "product_characteristic_float_value"]
pub struct NewProductCharacteristicFloatValue {
    pub value: BigDecimal,
}

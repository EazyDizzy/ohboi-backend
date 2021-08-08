use serde::Serialize;

use crate::schema::product_characteristic_string_value;

#[derive(Serialize, Queryable)]
pub struct ProductCharacteristicStringValue {
    pub id: i32,
    pub value: String,
}

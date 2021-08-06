use serde::Serialize;

use crate::schema::product_characteristic_int_value;

#[derive(Serialize, Queryable)]
pub struct ProductCharacteristicIntValue {
    pub id: i32,
    pub value: i32,
}

#[derive(Insertable)]
#[table_name = "product_characteristic_int_value"]
pub struct NewProductCharacteristicIntValue {
    pub value: i32,
}

use serde::Serialize;

use lib::schema::product_characteristic_string_value;

#[derive(Serialize, Queryable)]
pub struct ProductCharacteristicStringValue {
    pub id: i32,
    pub value: String,
}

#[derive(Insertable)]
#[table_name = "product_characteristic_string_value"]
pub struct NewProductCharacteristicStringValue {
    pub value: String,
}

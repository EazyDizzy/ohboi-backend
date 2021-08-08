use serde::Serialize;
use crate::schema::product_characteristic_enum_value;


#[derive(Serialize, Queryable)]
pub struct ProductCharacteristicEnumValue {
    pub id: i32,
    pub value: String,
}
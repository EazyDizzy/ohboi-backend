use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct ProductCharacteristic {
    pub id: i32,
    pub product_id: i32,
    pub characteristic_id: i32,
    pub value_id: i32,
}
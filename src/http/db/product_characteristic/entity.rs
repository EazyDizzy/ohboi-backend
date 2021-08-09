use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct ProductCharacteristic {
    pub product_id: i32,
    pub characteristic_id: i16,
    pub value_id: i32,
}

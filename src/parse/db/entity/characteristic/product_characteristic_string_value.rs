use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct ProductCharacteristicStringValue {
    pub id: i32,
    pub value: String,
}
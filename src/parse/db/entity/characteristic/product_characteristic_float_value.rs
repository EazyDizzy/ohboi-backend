use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct ProductCharacteristicFloatValue {
    pub id: i32,
    pub value: i16,
}
use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct ProductCharacteristicIntValue {
    pub id: i32,
    pub value: i16,
}
use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct ProductCharacteristicBoolValue {
    pub id: i32,
    pub value: bool,
}
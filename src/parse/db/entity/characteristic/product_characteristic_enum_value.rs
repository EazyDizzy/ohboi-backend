use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct ProductCharacteristicEnumValue {
    pub id: i32,
    pub value: i16,
}
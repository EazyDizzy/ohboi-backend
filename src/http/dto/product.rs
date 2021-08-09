use bigdecimal::BigDecimal;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ProductInfo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub lowest_price: BigDecimal,
    pub highest_price: BigDecimal,
    pub images: Option<Vec<String>>,
    pub category: i32,
    pub characteristics: ProductCharacteristicsMapped,
}

#[derive(Serialize, Debug)]
pub struct ProductCharacteristicsMapped {
    pub int: Vec<CharacteristicIntValue>,
    pub float: Vec<CharacteristicFloatValue>,
    pub string: Vec<CharacteristicStringValue>,
    pub enums: Vec<CharacteristicEnumValue>,
}
#[derive(Serialize, Debug)]
pub struct CharacteristicIntValue {
    pub characteristic_id: i16,
    pub value: i32,
}
#[derive(Serialize, Debug)]
pub struct CharacteristicFloatValue {
    pub characteristic_id: i16,
    pub value: f32,
}
#[derive(Serialize, Debug)]
pub struct CharacteristicStringValue {
    pub characteristic_id: i16,
    pub value: String,
}
#[derive(Serialize, Debug)]
pub struct CharacteristicEnumValue {
    pub characteristic_id: i16,
    pub value: String,
}

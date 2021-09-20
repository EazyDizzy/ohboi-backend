use std::fmt::{Debug, Display};

use bigdecimal::BigDecimal;
use serde::Deserialize;
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductCharacteristicsMapped {
    pub int: Vec<CharacteristicIntValue>,
    pub float: Vec<CharacteristicFloatValue>,
    pub string: Vec<CharacteristicStringValue>,
    pub enums: Vec<CharacteristicEnumValue>,
}
// TODO generic value
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacteristicIntValue {
    pub characteristic_id: i16,
    pub value: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacteristicFloatValue {
    pub characteristic_id: i16,
    pub value: f32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacteristicStringValue {
    pub characteristic_id: i16,
    pub value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacteristicEnumValue {
    pub characteristic_id: i16,
    pub value: String,
}

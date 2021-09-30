use std::fmt::Debug;

use bigdecimal::BigDecimal;
use serde::Deserialize;
use serde::Serialize;

use lib::dto::product_characteristic::{
    CharacteristicEnumValue, CharacteristicFloatValue, CharacteristicIntValue,
    CharacteristicStringValue,
};

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

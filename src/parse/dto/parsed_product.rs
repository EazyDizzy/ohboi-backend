use serde::Serialize;

use crate::parse::dto::characteristic::float_characteristic::FloatCharacteristic;
use crate::parse::dto::characteristic::int_characteristic::IntCharacteristic;
use crate::parse::dto::characteristic::string_characteristic::StringCharacteristic;
use crate::parse::dto::characteristic::enum_characteristic::EnumCharacteristic;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct LocalParsedProduct {
    pub title: String,
    pub price: f64,
    pub available: bool,
    pub external_id: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct InternationalParsedProduct {
    pub title: String,
    pub price: f64,
    pub original_price: f64,
    pub available: bool,
    pub external_id: String,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct AdditionalParsedProductInfo {
    pub image_urls: Vec<String>,
    pub description: String,
    pub available: bool,
    pub characteristics: Vec<TypedCharacteristic>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum TypedCharacteristic {
    Float(FloatCharacteristic),
    Int(IntCharacteristic),
    String(StringCharacteristic),
    Enum(EnumCharacteristic),
}

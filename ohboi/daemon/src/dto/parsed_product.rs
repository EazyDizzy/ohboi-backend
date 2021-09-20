use serde::Serialize;

use lib::dto::characteristic::TypedCharacteristic;

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

#[derive(Debug)]
pub struct AdditionalParsedProductInfo {
    pub image_urls: Vec<String>,
    pub description: String,
    pub available: bool,
    pub characteristics: Vec<TypedCharacteristic>,
}

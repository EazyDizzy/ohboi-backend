use serde::{Serialize};

#[derive(Serialize, Debug, PartialEq)]
pub struct ParsedProduct {
    pub title: String,
    pub price: f64,
    pub available: bool,
    pub external_id: String,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct AdditionalParsedProductInfo {
    pub image_url: String,
    pub description: String,
    pub available: bool,
}
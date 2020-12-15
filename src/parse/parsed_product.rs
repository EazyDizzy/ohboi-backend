use serde::{Serialize};

#[derive(Serialize, Debug, PartialEq)]
pub struct ParsedProduct {
    pub title: String,
    pub price: f64,
    pub available: bool,
    pub image_url: String
}
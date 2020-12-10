use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct ParsedProduct {
    pub title: String,
    pub price: f32,
    pub available: bool,
}
use crate::schema::product;

use chrono::NaiveDateTime;
use serde::{Serialize};
use bigdecimal::BigDecimal;

#[derive(Serialize, Queryable, Debug)]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "price")]
    pub lowest_price: BigDecimal,
    pub highest_price: BigDecimal,
    pub images: Option<Vec<String>>,
    pub category: i32,
    #[serde(skip)]
    pub enabled: bool,

    #[serde(skip)]
    pub created_at: NaiveDateTime,
    #[serde(skip)]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "product"]
pub struct NewProduct<'a> {
    pub category: i32,
    pub title: &'a str,
    pub lowest_price: BigDecimal,
    pub highest_price: BigDecimal,
    pub enabled: bool,

    pub created_at: &'a NaiveDateTime,
    pub updated_at: &'a NaiveDateTime,
}
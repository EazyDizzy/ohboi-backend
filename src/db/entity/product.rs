use crate::schema::product;
use chrono::NaiveDateTime;
use serde::{Serialize};
use bigdecimal::{BigDecimal};

#[derive(Serialize, Queryable)]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub description: String,
    #[serde(rename = "price")]
    pub lowest_price: BigDecimal,
    pub images: Vec<String>,

    #[serde(skip)]
    pub created_at: NaiveDateTime,
    #[serde(skip)]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "product"]
pub struct NewProduct<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub lowest_price: BigDecimal,
    pub images: &'a Vec<String>,
    pub created_at: &'a NaiveDateTime,
    pub updated_at: &'a NaiveDateTime,
}
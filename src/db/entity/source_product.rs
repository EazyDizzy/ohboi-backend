use crate::schema::source_product;

use chrono::NaiveDateTime;
use serde::{Serialize};
use bigdecimal::BigDecimal;

#[derive(Serialize, Queryable, Debug)]
pub struct SourceProduct {
    pub id: i32,
    pub source_id: i32,
    pub product_id: i32,
    pub price: BigDecimal,
    pub enabled: bool,
    pub external_id: String,

    #[serde(skip)]
    pub updated_at: NaiveDateTime,
}


#[derive(Insertable)]
#[table_name = "source_product"]
pub struct NewSourceProduct<'a> {
    pub source_id: i32,
    pub product_id: i32,
    pub price: BigDecimal,
    pub enabled: bool,
    pub external_id: &'a str,

    pub updated_at: &'a NaiveDateTime,
}
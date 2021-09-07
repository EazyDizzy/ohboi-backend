use lib::schema::source_product_price_history;

use chrono::NaiveDateTime;
use serde::{Serialize};
use bigdecimal::BigDecimal;

#[derive(Serialize, Queryable, Debug)]
pub struct SourceProductPriceHistory {
    pub id: i32,
    pub source_id: i32,
    pub product_id: i32,
    pub price: BigDecimal,
    pub external_id: String,

    #[serde(skip)]
    pub created_at: NaiveDateTime,
}


#[derive(Insertable)]
#[table_name = "source_product_price_history"]
pub struct NewSourceProductPriceHistory<'a> {
    pub source_id: i32,
    pub product_id: i32,
    pub price: BigDecimal,
    pub external_id: &'a str,

    pub created_at: &'a NaiveDateTime,
}
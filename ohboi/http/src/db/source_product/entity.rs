use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize, Queryable, Debug)]
pub struct SourceProduct {
    pub id: i32,
    pub source_id: i32,
    #[serde(skip)]
    pub product_id: i32,
    #[serde(skip)]
    pub external_id: String,
    pub price: BigDecimal,
    pub original_price: BigDecimal,
    #[serde(skip)]
    pub enabled: bool,

    #[serde(skip)]
    pub updated_at: NaiveDateTime,
}
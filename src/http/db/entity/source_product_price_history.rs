use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::Serialize;

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
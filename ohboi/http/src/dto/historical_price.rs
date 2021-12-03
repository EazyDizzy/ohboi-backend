use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::Serialize;

use lib::db::source_product_price_history::entity::SourceProductPriceHistory;

#[derive(Serialize, Debug)]
pub struct HistoricalPrice {
    pub source_id: i32,
    pub price: BigDecimal,
    pub date: NaiveDateTime,
}

impl From<SourceProductPriceHistory> for HistoricalPrice {
    fn from(entity: SourceProductPriceHistory) -> Self {
        HistoricalPrice {
            source_id: entity.source_id,
            price: entity.price,
            date: entity.created_at,
        }
    }
}

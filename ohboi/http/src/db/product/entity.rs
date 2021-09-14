use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::Serialize;
use lib::schema::product;

#[derive(Serialize, Queryable, QueryableByName, Debug)]
#[table_name = "product"]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
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
use crate::db::entity;
use crate::db;
use chrono::Utc;
use crate::db::entity::NewProduct;
use bigdecimal::BigDecimal;
use crate::schema::product;
use diesel::RunQueryDsl;

pub fn create(title: &str, price: f64) -> entity::Product {
    let connection = &db::establish_connection();
    let now = Utc::now();

    let new_product = NewProduct {
        title,

        description: "",
        lowest_price: BigDecimal::from(price),
        images: &vec![],
        created_at: &now.naive_utc(),
        updated_at: &now.naive_utc(),
    };

    diesel::insert_into(product::table)
        .values(&new_product)
        .get_result(connection)
        .expect("Error saving new product")
}
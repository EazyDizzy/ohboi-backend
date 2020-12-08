use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::db::repository;

pub async fn create(item: web::Json<Product>) -> HttpResponse {
    let created_product = repository::product::create(
        &item.title,
        &item.description,
        item.price,
        &item.images,
    );
    HttpResponse::Ok().json(created_product)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    title: String,
    description: String,
    images: Vec<String>,
    price: f64,
}
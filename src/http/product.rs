use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::db::repository::product::get_all_products_of_category;

pub async fn get_products(filters: web::Json<ProductFilters>) -> HttpResponse {
    let products = get_all_products_of_category(&filters.category, &filters.page);
    HttpResponse::Ok().json(products)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductFilters {
    category: i32,
    page: i32,
}

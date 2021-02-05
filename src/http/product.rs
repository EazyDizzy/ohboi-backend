use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

use crate::http::db::repository::product::get_all_products_of_category;

pub async fn get_products(filters: web::Json<ProductFilters>) -> HttpResponse {
    let products = get_all_products_of_category(&filters);
    HttpResponse::Ok().json(products)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductFilters {
    pub title: Option<String>,
    pub category: Option<Vec<i32>>,
    pub page: i32,
}

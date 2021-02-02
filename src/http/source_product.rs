use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::http::db::repository::source_product::get_all_for_product;

pub async fn get_source_products(filters: web::Json<SourceProductFilters>) -> HttpResponse {
    let products = get_all_for_product(&filters.id);

    HttpResponse::Ok().json(products)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceProductFilters {
    id: i32
}
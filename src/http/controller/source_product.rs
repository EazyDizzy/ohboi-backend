use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

use crate::http::db::source_product::repository::get_all_for_product;

pub async fn get_source_products(filters: web::Json<SourceProductFilters>) -> HttpResponse {
    let products = get_all_for_product(&filters.id);

    HttpResponse::Ok().json(products)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceProductFilters {
    id: i32
}
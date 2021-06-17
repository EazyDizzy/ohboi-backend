use actix_web::HttpResponse;
use actix_web_validator::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::http::db::source_product::repository::get_all_for_product;

pub async fn get_source_products(filters: Json<SourceProductFilters>) -> HttpResponse {
    let products = get_all_for_product(&filters.id);

    HttpResponse::Ok().json(products)
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SourceProductFilters {
    #[validate(range(min = 1, message = "should be bigger than zero"), range(max = 4294967295, message = "should be less than 4294967295"))]
    id: i32
}
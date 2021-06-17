use actix_web::HttpResponse;
use actix_web_validator::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::http::db::product::repository::get_all_products_of_category;

pub async fn get_products(filters: Json<ProductFilters>) -> HttpResponse {
    let products = get_all_products_of_category(&filters);
    HttpResponse::Ok().json(products)
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ProductFilters {
    #[validate(length(min = 1, max = 1000, message = "Title should have length from 1 to 1000"))]
    pub title: Option<String>,
    #[validate(length(min = 1, max = 1000, message = "Category should be an array of 1-1000 elements"))]
    pub category: Option<Vec<i32>>,
    #[validate(range(min = 0, message = "Page should be bigger than or equal to zero"), range(max = 4294967295, message = "Page should be less than 4294967295"))]
    pub page: u32,
}

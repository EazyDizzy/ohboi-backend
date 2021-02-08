use actix_web::HttpResponse;
use actix_web_validator::{Json, Validate};
use serde::{Deserialize, Serialize};

use crate::http::db::repository::product::get_all_products_of_category;

pub async fn get_products(filters: Json<ProductFilters>) -> HttpResponse {
    let products = get_all_products_of_category(&filters);
    HttpResponse::Ok().json(products)
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ProductFilters {
    #[validate(length(min = 1, max = 1000))]
    pub title: Option<String>,
    #[validate(length(min = 1, max = 1000))]
    pub category: Option<Vec<i32>>,
    #[validate(range(min = 0, message = "Page should be bigger than zero"), range(max = 4294967295, message = "Page should be less than 4294967295"))]
    pub page: u32,
}

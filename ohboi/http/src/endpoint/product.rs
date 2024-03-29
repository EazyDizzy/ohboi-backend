use actix_web::HttpResponse;
use actix_web_validator::{Json, Query};
use serde::{Deserialize, Serialize};
use validator::Validate;

use lib::db::repository::exchange_rate::try_get_exchange_rate_by_code;
use crate::db::product::repository::{get_filtered_products, get_product_info};
use crate::util::product::convert_product_prices;
use lib::my_enum::CurrencyEnum;
use crate::dto::product::ProductCharacteristicsMapped;

// TODO add hostname to the image urls to remove these dependency from fe
#[allow(clippy::needless_pass_by_value)]
pub fn get_product(params: Query<ProductParams>) -> HttpResponse {
    let product = get_product_info(&params);
    if product.is_none() {
        return HttpResponse::NotFound().json("Not found");
    }

    let product = product.unwrap();

    HttpResponse::Ok().json(product)
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ProductParams {
    #[validate(
        range(min = 0, message = "should be bigger than or equal to zero"),
        range(max = 4294967295, message = "should be less than 4294967295")
    )]
    pub id: i32,

    pub currency: CurrencyEnum,
}

#[allow(clippy::needless_pass_by_value)]
pub fn get_products(filters: Json<ProductFilters>) -> HttpResponse {
    let mut products = get_filtered_products(&filters.0);
    let rate = try_get_exchange_rate_by_code(filters.currency);

    for product in &mut products {
        convert_product_prices(product, rate);
    }

    HttpResponse::Ok().json(products)
}

#[derive(Debug, Serialize, Deserialize, Validate, Default)]
pub struct ProductFilters {
    #[validate(length(min = 1, max = 1000, message = "should have length from 1 to 1000"))]
    pub title: Option<String>,

    pub currency: CurrencyEnum,
    #[validate(length(min = 1, max = 1000, message = "should be an array of 1-1000 elements"))]
    pub category: Option<Vec<i32>>,
    #[validate(length(min = 1, max = 1000, message = "should be an array of 1-1000 elements"))]
    pub source: Option<Vec<i32>>,
    #[validate(
        range(min = 0, message = "should be bigger than or equal to zero"),
        range(max = 4294967295, message = "should be less than 4294967295")
    )]
    pub page: u32,

    #[validate(
        range(min = 0, message = "should be bigger than or equal to zero"),
        range(max = 4294967295, message = "should be less than 4294967295")
    )]
    pub min_price: Option<f64>,
    #[validate(
        range(min = 1, message = "should be bigger than or equal to zero"),
        range(max = 4294967295, message = "should be less than 4294967295")
    )]
    pub max_price: Option<f64>,

    pub characteristics: Option<ProductCharacteristicsMapped>,

    pub sort_by: Option<SearchSortKey>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SearchSortKey {
    PriceAsc,
    PriceDesc,
    UpdateDateAsc,
    UpdateDateDesc,
}
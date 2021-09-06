use actix_web::HttpResponse;

use crate::db::category::repository::get_all;

pub fn get_all_categories() -> HttpResponse {
    let categories = get_all();

    HttpResponse::Ok().json(categories)
}
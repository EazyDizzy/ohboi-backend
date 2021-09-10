use actix_web::HttpResponse;

use crate::db::source::repository::get_all_enabled;

pub fn get_all_sources() -> HttpResponse {
    let sources = get_all_enabled();

    HttpResponse::Ok().json(sources)
}
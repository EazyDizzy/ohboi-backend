use actix_web::HttpResponse;
use crate::http::db::repository::source::get_all_enabled;

pub fn get_all_sources() -> HttpResponse {
    let sources = get_all_enabled();

    HttpResponse::Ok().json(sources)
}
use actix_web::HttpResponse;

use lib::util::all_characteristics::get_all_characteristics_dto;

pub fn get_all_characteristics() -> HttpResponse {
    let chars = get_all_characteristics_dto();

    HttpResponse::Ok().json(chars)
}

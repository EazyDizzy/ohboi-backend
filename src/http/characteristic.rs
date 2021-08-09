use actix_web::HttpResponse;


pub fn get_all_characteristics() -> HttpResponse {
    let categories = get_all();

    HttpResponse::Ok().json(categories)
}
use actix_web::{get, web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[get("/{id}/{name}/index.html")]
pub async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

pub async fn create(item: web::Json<User>) -> HttpResponse {
    HttpResponse::Ok().json(item.0)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
}
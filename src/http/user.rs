use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::http::db::repository;

pub async fn create(item: web::Json<User>) -> HttpResponse {
    let created_user = repository::user::create(&item.username);
    HttpResponse::Ok().json(created_user)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
}
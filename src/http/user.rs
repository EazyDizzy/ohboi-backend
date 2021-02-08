use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

use crate::http::db::user;

pub async fn create(item: web::Json<User>) -> HttpResponse {
    let created_user = user::repository::create(&item.username);
    HttpResponse::Ok().json(created_user)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
}
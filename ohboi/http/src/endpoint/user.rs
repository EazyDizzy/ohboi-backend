use actix_web::HttpResponse;
use actix_web_validator::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::db::user;

#[allow(clippy::needless_pass_by_value)]
pub fn create(item: Json<User>) -> HttpResponse {
    let created_user = user::repository::create(&item.username);
    HttpResponse::Ok().json(created_user)
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct User {
    #[validate(length(min = 1, max = 1000, message = "should have length from 1 to 1000"))]
    username: String,
}

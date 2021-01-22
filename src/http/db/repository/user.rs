use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::http::db;
use crate::http::db::entity;
use crate::schema::users;

pub fn create(username: &str) -> entity::User {
    let connection = &db::establish_connection();
    let now = Utc::now();

    let new_user = entity::NewUser {
        username,
        created_at: &now.naive_utc(),
        updated_at: &now.naive_utc(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(connection)
        .expect("Error saving new user")
}

pub fn get_by_id(user_id: &i32) -> entity::User {
    use crate::schema::users::dsl::*;

    let connection = &db::establish_connection();

    let target = users.filter(id.eq(user_id));
    let results: Vec<entity::User> = target
        .limit(1)
        .load::<entity::User>(connection)
        .expect("Error loading product");

    // TODO what?
    let user = results.into_iter().next();

    user.unwrap()
}
use chrono::Utc;
use crate::schema::users;
use diesel::{RunQueryDsl};
use crate::db;
use crate::db::entity;

pub fn create_post(username: &str) -> entity::User {
    let connection = &db::establish_connection();
    let now = Utc::now();

    let new_post = entity::NewUser {
        username,
        created_at: &now.naive_utc(),
        updated_at: &now.naive_utc(),
    };

    diesel::insert_into(users::table)
        .values(&new_post)
        .get_result(connection)
        .expect("Error saving new user")
}
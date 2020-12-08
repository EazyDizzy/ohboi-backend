use crate::schema::users;
use crate::db;
use crate::db::entity;
use chrono::Utc;
use diesel::{RunQueryDsl};

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
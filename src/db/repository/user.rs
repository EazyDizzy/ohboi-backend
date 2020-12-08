use chrono::Utc;
use crate::models::{NewUser, User};
use crate::schema::users;
use diesel::{RunQueryDsl};
use crate::db;

pub fn create_post(username: &str) -> User {
    let connection = &db::establish_connection();
    let now = Utc::now();

    let new_post = NewUser {
        username,
        created_at: &now.naive_utc(),
        updated_at: &now.naive_utc(),
    };

    diesel::insert_into(users::table)
        .values(&new_post)
        .get_result(connection)
        .expect("Error saving new user")
}
#[macro_use]
extern crate diesel;
extern crate dotenv;

use self::diesel::prelude::*;
use chrono::Utc;

pub fn create_post(conn: &PgConnection, username: &str) -> User {
    use schema::users;

    let now = Utc::now();

    let new_post = NewUser {
        username,
        created_at: &now.naive_utc(),
        updated_at: &now.naive_utc(),
    };

    diesel::insert_into(users::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new user")
}
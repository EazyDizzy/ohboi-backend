#[macro_use]
extern crate diesel;
extern crate dotenv;

mod http;
mod db;
mod schema;
mod models;

use self::diesel::prelude::*;
use chrono::Utc;

fn main() {
    let connection = db::establish_connection();

    create_post(&connection, "her");

    let result = http::run_server();
    match result {
        Ok(_) => println!("Server started."),
        Err(e) => println!("Server failed: {}", e)
    }
}

use self::models::{User, NewUser};

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
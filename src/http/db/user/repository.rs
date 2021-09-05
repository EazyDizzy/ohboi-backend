use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use lib::db;
use crate::http::db::user::entity::{NewUser, User};
use lib::schema::users;

pub fn create(username: &str) -> User {
    let connection = &db::establish_connection();
    let now = Utc::now();

    let new_user = NewUser {
        username,
        created_at: &now.naive_utc(),
        updated_at: &now.naive_utc(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(connection)
        .expect("Error saving new user")
}

pub fn get_by_id(user_id: i32) -> User {
    use lib::schema::users::dsl::{id, users};

    let connection = &db::establish_connection();

    let target = users.filter(id.eq(user_id));
    let results: Vec<User> = target
        .limit(1)
        .load::<User>(connection)
        .expect("Error loading product");

    let user = results.into_iter().next();

    user.unwrap()
}
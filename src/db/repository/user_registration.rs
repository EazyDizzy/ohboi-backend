use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::db;
use crate::db::entity;
use crate::db::entity::UserRegistration;
use crate::db::repository::user::{create, get_by_id};
use crate::schema::user_registration;
use crate::my_enum::UserRegistrationType;

pub fn get_user_by_auth(expected_registration_type: &UserRegistrationType,
                        expected_email: &str,
                        expected_full_name: &str) -> entity::User {
    use crate::schema::user_registration::dsl::*;

    let connection = &db::establish_connection();

    let target = user_registration.filter(
        registration_type.eq(expected_registration_type)
                         .and(email.eq(expected_email))
                         .and(full_name.eq(expected_full_name)));
    let results: Vec<UserRegistration> = target
        .limit(1)
        .load::<UserRegistration>(connection)
        .expect("Error loading user_registration");

    if results.len() == 0 {
        let user = create(expected_full_name);
        create_registration(&user.id, expected_registration_type, expected_email, expected_full_name);
        user
    } else {
        let existing_user_registration = results.into_iter().next().unwrap();
        get_by_id(&existing_user_registration.user_id)
    }
}

fn create_registration(new_user_id: &i32,
                       registration_type: &UserRegistrationType,
                       email: &str,
                       full_name: &str) -> entity::UserRegistration {
    let connection = &db::establish_connection();

    let new_user_registration = entity::NewUserRegistration {
        user_id: new_user_id,
        registration_type,
        email,
        full_name,
    };

    diesel::insert_into(user_registration::table)
        .values(&new_user_registration)
        .get_result(connection)
        .expect("Error saving new user")
}
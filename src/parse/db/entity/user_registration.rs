use crate::my_enum::UserRegistrationType;
use crate::schema::user_registration;

#[derive(Queryable)]
pub struct UserRegistration {
    pub id: i32,
    pub user_id: i32,
    pub registration_type: UserRegistrationType,

    pub email: String,
    pub full_name: String,
}

#[derive(Insertable)]
#[table_name = "user_registration"]
pub struct NewUserRegistration<'a> {
    pub user_id: &'a i32,
    pub registration_type: &'a UserRegistrationType,

    pub email: &'a str,
    pub full_name: &'a str,
}
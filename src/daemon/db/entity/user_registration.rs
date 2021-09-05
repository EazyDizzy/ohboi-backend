use lib::my_enum::UserRegistrationType;

#[derive(Queryable)]
pub struct UserRegistration {
    pub id: i32,
    pub user_id: i32,
    pub registration_type: UserRegistrationType,

    pub email: String,
    pub full_name: String,
}
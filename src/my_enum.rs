#[derive(diesel_derive_enum::DbEnum, Debug)]
#[DieselType = "User_registration_type"]
pub enum UserRegistrationType {
    Google,
    Facebook,
    Apple,
}
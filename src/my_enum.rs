use serde::Serialize;

#[derive(diesel_derive_enum::DbEnum, Debug)]
#[DieselType = "User_registration_type"]
pub enum UserRegistrationType {
    Google,
    Facebook,
    Apple,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize)]
#[DieselType = "Currency_enum"]
pub enum CurrencyEnum {
    EUR,
    RUB,
    UAH,
    USD,
}
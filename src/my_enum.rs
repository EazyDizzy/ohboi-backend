use serde::{Deserialize, Serialize};

#[derive(diesel_derive_enum::DbEnum, Debug)]
#[DieselType = "User_registration_type"]
pub enum UserRegistrationType {
    Google,
    Facebook,
    Apple,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize, Copy, Clone)]
#[DieselType = "Currency_enum"]
pub enum CurrencyEnum {
    EUR,
    RUB,
    UAH,
    USD,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize, Copy, Clone)]
#[DieselType = "Characteristic_value_type"]
pub enum CharacteristicValueType {
    Float,
    Int,
    String,
    EnumString,
    Bool,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize, Copy, Clone)]
#[DieselType = "Characteristic_visualisation_type"]
pub enum CharacteristicVisualisationType {
    Range,
    MultiSelector,
    SingleSelector,
    Bool,
}
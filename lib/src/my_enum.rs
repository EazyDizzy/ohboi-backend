use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

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

impl Default for CurrencyEnum {
    fn default() -> Self {
        CurrencyEnum::EUR
    }
}

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize, Copy, Clone)]
#[DieselType = "Characteristic_value_type"]
pub enum CharacteristicValueType {
    Float,
    Int,
    String,
    Enum,
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

#[derive(diesel_derive_enum::DbEnum, EnumIter, Serialize, Deserialize, Debug, Copy, Clone)]
#[DieselType = "Characteristic_group_slug"]
pub enum CharacteristicGroupSlug {
    Processor,
    Memory,
    Connection,
    Display,
    Camera,
    Sensors,
    Power,
    Appearance,
    General,
}

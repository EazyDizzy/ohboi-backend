use serde::Deserialize;
use serde::Serialize;

// TODO generic value
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacteristicIntValue {
    pub characteristic_id: i16,
    pub value: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacteristicFloatValue {
    pub characteristic_id: i16,
    pub value: f32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacteristicStringValue {
    pub characteristic_id: i16,
    pub value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacteristicEnumValue {
    pub characteristic_id: i16,
    pub value: String,
}

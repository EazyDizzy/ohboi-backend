use serde::Serialize;

use crate::dto::characteristic::enum_characteristic::EnumCharacteristic;
use crate::dto::characteristic::float_characteristic::FloatCharacteristic;
use crate::dto::characteristic::int_characteristic::IntCharacteristic;
use crate::dto::characteristic::string_characteristic::StringCharacteristic;

pub mod enum_characteristic;
pub mod float_characteristic;
pub mod int_characteristic;
pub mod string_characteristic;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum TypedCharacteristic {
    Float(FloatCharacteristic),
    Int(IntCharacteristic),
    String(StringCharacteristic),
    Enum(EnumCharacteristic),
}

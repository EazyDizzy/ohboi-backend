use serde::Serialize;

use crate::common::dto::characteristic::enum_characteristic::EnumCharacteristic;
use crate::common::dto::characteristic::float_characteristic::FloatCharacteristic;
use crate::common::dto::characteristic::int_characteristic::IntCharacteristic;
use crate::common::dto::characteristic::string_characteristic::StringCharacteristic;

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

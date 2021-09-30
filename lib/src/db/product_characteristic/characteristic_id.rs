use strum::IntoEnumIterator;
use strum::VariantNames;

use crate::dto::characteristic::enum_characteristic::EnumCharacteristic;
use crate::dto::characteristic::float_characteristic::FloatCharacteristic;
use crate::dto::characteristic::int_characteristic::IntCharacteristic;
use crate::dto::characteristic::string_characteristic::StringCharacteristic;
use crate::dto::characteristic::TypedCharacteristic;
use crate::util::characteristic_id::get_characteristic_id;

pub fn get_characteristic_by_id(char_id: i16) -> Option<TypedCharacteristic> {
    for item in FloatCharacteristic::iter() {
        if get_characteristic_id(&TypedCharacteristic::Float(item)) == char_id {
            return Some(TypedCharacteristic::Float(item));
        }
    }
    for item in IntCharacteristic::iter() {
        if get_characteristic_id(&TypedCharacteristic::Int(item)) == char_id {
            return Some(TypedCharacteristic::Int(item));
        }
    }
    for item in StringCharacteristic::iter() {
        if get_characteristic_id(&TypedCharacteristic::String(item.clone())) == char_id {
            return Some(TypedCharacteristic::String(item));
        }
    }

    for variant in EnumCharacteristic::VARIANTS {
        let item = EnumCharacteristic::type_from_name(variant);
        let en = TypedCharacteristic::Enum(item);

        if get_characteristic_id(&en) == char_id {
            return Some(en);
        }
    }

    None
}

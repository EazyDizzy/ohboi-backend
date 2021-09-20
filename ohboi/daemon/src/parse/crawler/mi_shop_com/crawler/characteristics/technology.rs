use lib::dto::characteristic::enum_characteristic::{EnumCharacteristic, Technology};

use crate::parse::crawler::characteristic_parser::*;

pub fn extract_technology_characteristic(
    title: &str,
    value: &str,
    context: CharacteristicParsingContext,
) -> Option<EnumCharacteristic> {
    let characteristic = match title {
        "NFC" => {
            bool_value(&context, value).and_then(|v| if v { Some(Technology::NFC) } else { None })
        }
        "Автофокус" => bool_value(&context, value).and_then(|v| {
            if v {
                Some(Technology::Autofocus)
            } else {
                None
            }
        }),
        "Быстрая зарядка" => bool_value(&context, value).and_then(|v| {
            if v {
                Some(Technology::FastCharging)
            } else {
                None
            }
        }),
        "ИК-порт" => bool_value(&context, value).and_then(|v| {
            if v {
                Some(Technology::InfraredPort)
            } else {
                None
            }
        }),
        "Беспроводная зарядка" => bool_value(&context, value).and_then(|v| {
            if v {
                Some(Technology::WirelessCharger)
            } else {
                None
            }
        }),
        _ => None,
    };

    characteristic.map(|v| EnumCharacteristic::TechnologySupport(v))
}

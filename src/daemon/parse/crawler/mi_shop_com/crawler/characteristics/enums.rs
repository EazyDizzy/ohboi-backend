use regex::Regex;

use lib::dto::characteristic::enum_characteristic::EnumCharacteristic;
use crate::daemon::parse::crawler::characteristic_parser::*;
use crate::daemon::parse::crawler::mi_shop_com::crawler::media_format_parser::multiple_string_media_format_value;

lazy_static! {
    static ref NO_DESCRIPTION_RE: Regex = Regex::new(r"(?ms)[A-Za-z./ 0-9\-+–]{2,}").unwrap();
}

pub fn extract_enum_characteristic(
    title: &str,
    value: &str,
    context: CharacteristicParsingContext,
) -> Vec<EnumCharacteristic> {
    match extract_single_enum_characteristic(title, value, &context) {
        Some(v) => {
            vec![v]
        }
        None => extract_multiple_enum_characteristic(title, value, &context),
    }
}

fn extract_single_enum_characteristic(
    title: &str,
    value: &str,
    context: &CharacteristicParsingContext,
) -> Option<EnumCharacteristic> {
    match title {
        "Тип разъема для зарядки" => {
            parse_and_capture(&context, &value, enum_charging_connector_type_value)
                .and_then(|v| Some(EnumCharacteristic::ChargingConnectorType(v)))
        }
        "Слот для карты памяти" => {
            parse_and_capture(&context, &value, enum_memory_card_slot_value)
                .and_then(|v| Some(EnumCharacteristic::MemoryCardSlot(v)))
        }
        "Страна производитель" => {
            parse_and_capture(&context, &value, enum_country_value)
                .and_then(|v| Some(EnumCharacteristic::ProducingCountry(v)))
        }
        "Аудиоразъем" | "Вход аудио" => {
            if let Some(value) = NO_DESCRIPTION_RE.captures_iter(value).next() {
                parse_and_capture(
                    &context,
                    &value.get(0).unwrap().as_str(),
                    enum_audio_jack_value,
                )
                .and_then(|v| Some(EnumCharacteristic::AudioJack(v)))
            } else {
                None
            }
        }
        "Аккумулятор" => parse_and_capture(&context, &value, enum_battery_type_value)
            .and_then(|v| Some(EnumCharacteristic::BatteryType(v))),
        "Тип дисплея" => parse_and_capture(&context, &value, enum_display_type_value)
            .and_then(|v| Some(EnumCharacteristic::DisplayType(v))),
        _ => None,
    }
}

fn extract_multiple_enum_characteristic(
    title: &str,
    value: &str,
    context: &CharacteristicParsingContext,
) -> Vec<EnumCharacteristic> {
    match title {
        "SIM-карта" => multiple_parse_and_capture(&context, value, enum_sim_card_value)
            .into_iter()
            .map(|v| EnumCharacteristic::SimCard(v))
            .collect(),
        "Поддерживаемые медиа форматы" => {
            multiple_string_media_format_value(&context, value)
                .into_iter()
                .map(|v| EnumCharacteristic::SupportedMediaFormat(v))
                .collect()
        }
        "Интернет" => {
            multiple_parse_and_capture(&context, value, enum_internet_connection_technology_value)
                .into_iter()
                .map(|v| EnumCharacteristic::InternetConnectionTechnology(v))
                .collect()
        }
        "Спутниковая навигация" => {
            multiple_parse_and_capture(&context, value, enum_satellite_navigation_value)
                .into_iter()
                .map(|v| EnumCharacteristic::SatelliteNavigation(v))
                .collect()
        }
        "Wi-Fi (802.11)" => multiple_parse_and_capture(&context, value, enum_wifi_standard_value)
            .into_iter()
            .map(|v| EnumCharacteristic::WifiStandard(v))
            .collect(),
        "Материал" => multiple_parse_and_capture(&context, value, enum_material_value)
            .into_iter()
            .map(|v| EnumCharacteristic::Material(v))
            .collect(),
        _ => vec![],
    }
}

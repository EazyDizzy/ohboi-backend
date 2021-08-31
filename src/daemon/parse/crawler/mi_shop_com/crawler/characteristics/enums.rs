use regex::Regex;

use crate::common::dto::characteristic::enum_characteristic::EnumCharacteristic;
use crate::daemon::parse::crawler::characteristic_parser::*;
use crate::daemon::parse::crawler::crawler::Crawler;
use crate::daemon::parse::crawler::mi_shop_com::crawler::media_format_parser::multiple_string_media_format_value;
use crate::daemon::parse::crawler::mi_shop_com::crawler::MiShopComCrawler;

lazy_static! {
    static ref NO_DESCRIPTION_RE: Regex = Regex::new(r"(?ms)[A-Za-z./ 0-9\-+–]{2,}").unwrap();
}

pub fn extract_enum_characteristics(
    crawler: &MiShopComCrawler,
    external_id: &str,
    titles: &Vec<String>,
    values: &Vec<String>,
) -> (Vec<EnumCharacteristic>, Vec<usize>) {
    let mut characteristics: Vec<EnumCharacteristic> = vec![];
    let mut parsed_indexes = vec![];

    for (title_index, title) in titles.into_iter().enumerate() {
        let value = values.get(title_index).unwrap();
        let context = CharacteristicParsingContext {
            title: &title,
            external_id,
            source: crawler.get_source(),
        };

        match title.as_str() {
            "SIM-карта" => {
                multiple_parse_and_capture(&context, value, enum_sim_card_value)
                    .into_iter()
                    .for_each(|v| characteristics.push(EnumCharacteristic::SimCard(v)));
                parsed_indexes.push(title_index);
            }
            "Поддерживаемые медиа форматы" => {
                multiple_string_media_format_value(&context, value)
                    .into_iter()
                    .for_each(|v| {
                        characteristics.push(EnumCharacteristic::SupportedMediaFormat(v))
                    });
                parsed_indexes.push(title_index);
            }
            "Интернет" => {
                multiple_parse_and_capture(
                    &context,
                    value,
                    enum_internet_connection_technology_value,
                )
                .into_iter()
                .for_each(|v| {
                    characteristics.push(EnumCharacteristic::InternetConnectionTechnology(v))
                });
                parsed_indexes.push(title_index);
            }
            "Спутниковая навигация" => {
                multiple_parse_and_capture(&context, value, enum_satellite_navigation_value)
                    .into_iter()
                    .for_each(|v| characteristics.push(EnumCharacteristic::SatelliteNavigation(v)));
                parsed_indexes.push(title_index);
            }
            "Wi-Fi (802.11)" => {
                multiple_parse_and_capture(&context, value, enum_wifi_standard_value)
                    .into_iter()
                    .for_each(|v| characteristics.push(EnumCharacteristic::WifiStandard(v)));
                parsed_indexes.push(title_index);
            }
            "Материал" => {
                multiple_parse_and_capture(&context, value, enum_material_value)
                    .into_iter()
                    .for_each(|v| characteristics.push(EnumCharacteristic::Material(v)));
                parsed_indexes.push(title_index);
            }
            _ => (),
        }
    }

    for (title_index, title) in titles.into_iter().enumerate() {
        let context = CharacteristicParsingContext {
            title: &title,
            external_id,
            source: crawler.get_source(),
        };
        let value = values.get(title_index).unwrap();
        let characteristic: Option<EnumCharacteristic> = match title.as_str() {
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
            "Аккумулятор" => {
                parse_and_capture(&context, &value, enum_battery_type_value)
                    .and_then(|v| Some(EnumCharacteristic::BatteryType(v)))
            }
            "Тип дисплея" => parse_and_capture(&context, &value, enum_display_type_value)
                .and_then(|v| Some(EnumCharacteristic::DisplayType(v))),
            _ => None,
        };

        if let Some(characteristic) = characteristic {
            parsed_indexes.push(title_index);
            characteristics.push(characteristic);
        }
    }

    (characteristics, parsed_indexes)
}

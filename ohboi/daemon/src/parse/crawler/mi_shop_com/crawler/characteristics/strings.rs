use lib::dto::characteristic::string_characteristic::StringCharacteristic;
use crate::parse::crawler::characteristic_parser::*;

pub fn extract_string_characteristic(title: &str, value: &str, context: CharacteristicParsingContext) -> Option<StringCharacteristic> {
    match title {
        "Процессор" => Some(StringCharacteristic::Processor(string_value(&value))),
        "Модель" => Some(StringCharacteristic::Model(string_value(&value))),
        "Контрастность" => Some(StringCharacteristic::Contrast(string_value(&value))),
        "Соотношение сторон" => {
            Some(StringCharacteristic::AspectRatio(string_value(&value)))
        }
        "Разрешение дисплея" => Some(StringCharacteristic::DisplayResolution(
            string_value(&value),
        )),
        "Видеопроцессор" => {
            Some(StringCharacteristic::VideoProcessor(string_value(&value)))
        }
        _ => None,
    }
}

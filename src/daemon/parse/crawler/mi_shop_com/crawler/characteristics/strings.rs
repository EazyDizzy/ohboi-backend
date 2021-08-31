use crate::common::dto::characteristic::string_characteristic::StringCharacteristic;
use crate::daemon::parse::crawler::characteristic_parser::*;

pub fn extract_string_characteristics(
    titles: &Vec<String>,
    values: &Vec<String>,
) -> (Vec<StringCharacteristic>, Vec<usize>) {
    let mut characteristics: Vec<StringCharacteristic> = vec![];
    let mut parsed_indexes = vec![];

    for (title_index, title) in titles.into_iter().enumerate() {
        let value = values.get(title_index).unwrap();
        let characteristic: Option<StringCharacteristic> = match title.as_str() {
            "Процессор" => Some(StringCharacteristic::Processor(string_value(&value))),
            "Модель" => Some(StringCharacteristic::Model(string_value(&value))),
            "Контрастность" => {
                Some(StringCharacteristic::Contrast(string_value(&value)))
            }
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
        };

        if let Some(characteristic) = characteristic {
            parsed_indexes.push(title_index);
            characteristics.push(characteristic);
        }
    }

    (characteristics, parsed_indexes)
}

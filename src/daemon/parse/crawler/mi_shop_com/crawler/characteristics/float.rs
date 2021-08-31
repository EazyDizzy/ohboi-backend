use crate::daemon::parse::crawler::characteristic_parser::*;
use crate::daemon::parse::crawler::crawler::Crawler;
use crate::daemon::parse::crawler::mi_shop_com::crawler::MiShopComCrawler;
use crate::common::dto::characteristic::float_characteristic::FloatCharacteristic;

pub fn extract_float_characteristics(
    crawler: &MiShopComCrawler,
    external_id: &str,
    titles: &Vec<String>,
    values: &Vec<String>,
) -> (Vec<FloatCharacteristic>, Vec<usize>) {
    let mut characteristics: Vec<FloatCharacteristic> = vec![];
    let mut parsed_indexes = vec![];

    for (title_index, title) in titles.into_iter().enumerate() {
        let value = values.get(title_index).unwrap();
        let context = CharacteristicParsingContext {
            title: &title,
            external_id,
            source: crawler.get_source(),
        };

        let characteristic: Option<FloatCharacteristic> =
            match title.as_str() {
                "Толщина (мм)" => float_value(&context, &value)
                    .and_then(|v| Some(FloatCharacteristic::Thickness_mm(v))),
                "Апертура" => float_aperture_value(&context, &value)
                    .and_then(|v| Some(FloatCharacteristic::Aperture(v))),
                "Ширина (мм)" => float_value(&context, &value)
                    .and_then(|v| Some(FloatCharacteristic::Width_mm(v))),
                "Высота (мм)" => float_value(&context, &value)
                    .and_then(|v| Some(FloatCharacteristic::Height_mm(v))),
                "Диагональ экрана" => float_diagonal_value(&context, &value)
                    .and_then(|v| Some(FloatCharacteristic::ScreenDiagonal(v))),
                "Bluetooth" => float_value(&context, &value)
                    .and_then(|v| Some(FloatCharacteristic::BluetoothVersion(v))),
                "Частота" => float_ghz_value(&context, &value)
                    .and_then(|v| Some(FloatCharacteristic::CPUFrequency_Ghz(v))),
                "Вес (г)" => float_value(&context, &value)
                    .and_then(|v| Some(FloatCharacteristic::Weight_gr(v))),
                "Версия MIUI" => float_miui_version_value(&context, &value)
                    .and_then(|v| Some(FloatCharacteristic::MIUIVersion(v))),
                "Версия Android" => float_android_version_value(&context, &value)
                    .and_then(|v| Some(FloatCharacteristic::AndroidVersion(v))),
                _ => None,
            };

        if let Some(characteristic) = characteristic {
            parsed_indexes.push(title_index);
            characteristics.push(characteristic);
        }
    }

    (characteristics, parsed_indexes)
}

use lib::dto::characteristic::float_characteristic::FloatCharacteristic;

use crate::parse::crawler::characteristic_parser::*;

pub fn extract_float_characteristic(
    title: &str,
    value: &str,
    context: CharacteristicParsingContext,
) -> Option<FloatCharacteristic> {
    match title {
        "Толщина (мм)" => {
            float_value(&context, value).map(FloatCharacteristic::Thickness_mm)
        }
        "Апертура" => {
            float_aperture_value(&context, value).map(FloatCharacteristic::Aperture)
        }
        "Ширина (мм)" => {
            float_value(&context, value).map(FloatCharacteristic::Width_mm)
        }
        "Высота (мм)" => {
            float_value(&context, value).map(FloatCharacteristic::Height_mm)
        }
        "Диагональ экрана" => {
            float_diagonal_value(&context, value).map(FloatCharacteristic::ScreenDiagonal)
        }
        "Bluetooth" => {
            float_value(&context, value).map(FloatCharacteristic::BluetoothVersion)
        }
        "Частота" => {
            float_ghz_value(&context, value).map(FloatCharacteristic::CPUFrequency_Ghz)
        }
        "Вес (г)" => float_value(&context, value).map(FloatCharacteristic::Weight_gr),
        "Версия MIUI" => {
            float_miui_version_value(&context, value).map(FloatCharacteristic::MIUIVersion)
        }
        "Версия Android" => float_android_version_value(&context, value)
            .map(FloatCharacteristic::AndroidVersion),
        _ => None,
    }
}

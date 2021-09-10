use lib::dto::characteristic::float_characteristic::FloatCharacteristic;
use crate::parse::crawler::characteristic_parser::*;

pub fn extract_float_characteristic(title: &str, value: &str, context: CharacteristicParsingContext) -> Option<FloatCharacteristic> {
    match title {
        "Толщина (мм)" => {
            float_value(&context, &value).and_then(|v| Some(FloatCharacteristic::Thickness_mm(v)))
        }
        "Апертура" => float_aperture_value(&context, &value)
            .and_then(|v| Some(FloatCharacteristic::Aperture(v))),
        "Ширина (мм)" => {
            float_value(&context, &value).and_then(|v| Some(FloatCharacteristic::Width_mm(v)))
        }
        "Высота (мм)" => {
            float_value(&context, &value).and_then(|v| Some(FloatCharacteristic::Height_mm(v)))
        }
        "Диагональ экрана" => float_diagonal_value(&context, &value)
            .and_then(|v| Some(FloatCharacteristic::ScreenDiagonal(v))),
        "Bluetooth" => float_value(&context, &value)
            .and_then(|v| Some(FloatCharacteristic::BluetoothVersion(v))),
        "Частота" => float_ghz_value(&context, &value)
            .and_then(|v| Some(FloatCharacteristic::CPUFrequency_Ghz(v))),
        "Вес (г)" => {
            float_value(&context, &value).and_then(|v| Some(FloatCharacteristic::Weight_gr(v)))
        }
        "Версия MIUI" => float_miui_version_value(&context, &value)
            .and_then(|v| Some(FloatCharacteristic::MIUIVersion(v))),
        "Версия Android" => float_android_version_value(&context, &value)
            .and_then(|v| Some(FloatCharacteristic::AndroidVersion(v))),
        _ => None,
    }
}

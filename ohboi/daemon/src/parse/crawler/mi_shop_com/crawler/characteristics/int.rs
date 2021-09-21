use lib::dto::characteristic::int_characteristic::IntCharacteristic;

use crate::parse::crawler::characteristic_parser::{
    int_fps_value, int_guarantee_value, int_hz_value, int_ma_h_value, int_memory_value,
    int_mp_value, int_nit_value, int_value, multiple_int_value, pix_int_value,
    CharacteristicParsingContext,
};

pub fn extract_int_characteristic(
    title: &str,
    value: &str,
    context: &CharacteristicParsingContext,
) -> Vec<IntCharacteristic> {
    match extract_single_int_characteristic(title, value, context) {
        Some(v) => {
            vec![v]
        }
        None => extract_multiple_int_characteristic(title, value, context),
    }
}

fn extract_single_int_characteristic(
    title: &str,
    value: &str,
    context: &CharacteristicParsingContext,
) -> Option<IntCharacteristic> {
    match title {
        "Количество ядер процессора" => {
            int_value(context, value).map(IntCharacteristic::NumberOfProcessorCores)
        }
        "Гарантия (мес)" => {
            int_guarantee_value(context, value).map(IntCharacteristic::Warranty_month)
        }
        "Встроенная память (ГБ)" => {
            int_value(context, value).map(IntCharacteristic::BuiltInMemory_GB)
        }
        "Оперативная память (ГБ)" => {
            int_value(context, value).map(IntCharacteristic::Ram_GB)
        }
        "Фронтальная камера (Мп)" => {
            int_mp_value(context, value).map(IntCharacteristic::FrontCamera_MP)
        }
        "Разрешение видеосъемки (пикс)" => {
            pix_int_value(context, value).map(IntCharacteristic::VideoResolution_Pix)
        }
        "Емкость аккумулятора (мА*ч)" => {
            int_ma_h_value(context, value).map(IntCharacteristic::BatteryCapacity_mA_h)
        }
        "Кол-во SIM-карт" => {
            int_value(context, value).map(IntCharacteristic::AmountOfSimCards)
        }
        "Частота кадров видеосъемки" => {
            int_fps_value(context, value).map(IntCharacteristic::Fps)
        }
        "Плотность пикселей (PPI)" => {
            int_value(context, value).map(IntCharacteristic::PPI)
        }
        "Максимальный объем карты памяти" => {
            int_memory_value(context, value).map(IntCharacteristic::MaxMemoryCardSize_GB)
        }
        "Яркость (кд/м²)" => {
            int_nit_value(context, value).map(IntCharacteristic::Brightness_cd_m2)
        }
        "Частота обновления" => {
            int_hz_value(context, value).map(IntCharacteristic::UpdateFrequency_Hz)
        }
        "Фотокамера (Мп)" => {
            int_mp_value(context, value).map(IntCharacteristic::Camera_mp)
        }
        _ => None,
    }
}

fn extract_multiple_int_characteristic(
    title: &str,
    value: &str,
    context: &CharacteristicParsingContext,
) -> Vec<IntCharacteristic> {
    match title {
        "Диапазоны LTE" => multiple_int_value(context, value)
            .into_iter()
            .map(IntCharacteristic::LTEDiapason)
            .collect(),
        "Диапазоны GSM" => multiple_int_value(context, value)
            .into_iter()
            .map(IntCharacteristic::GSMDiapason)
            .collect(),
        "Диапазоны UMTS" => multiple_int_value(context, value)
            .into_iter()
            .map(IntCharacteristic::UMTSDiapason)
            .collect(),
        _ => vec![],
    }
}

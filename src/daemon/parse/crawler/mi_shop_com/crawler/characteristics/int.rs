use crate::common::dto::characteristic::int_characteristic::IntCharacteristic;
use crate::daemon::parse::crawler::characteristic_parser::*;

pub fn extract_int_characteristic(
    title: &str,
    value: &str,
    context: CharacteristicParsingContext,
) -> Vec<IntCharacteristic> {
    match extract_single_int_characteristic(title, value, &context) {
        Some(v) => {
            vec![v]
        }
        None => extract_multiple_int_characteristic(title, value, &context),
    }
}

fn extract_single_int_characteristic(
    title: &str,
    value: &str,
    context: &CharacteristicParsingContext,
) -> Option<IntCharacteristic> {
    match title {
        "Количество ядер процессора" => int_value(context, &value)
            .and_then(|v| Some(IntCharacteristic::NumberOfProcessorCores(v))),
        "Гарантия (мес)" => int_guarantee_value(context, &value)
            .and_then(|v| Some(IntCharacteristic::Warranty_month(v))),
        "Встроенная память (ГБ)" => {
            int_value(context, &value).and_then(|v| Some(IntCharacteristic::BuiltInMemory_GB(v)))
        }
        "Оперативная память (ГБ)" => {
            int_value(context, &value).and_then(|v| Some(IntCharacteristic::Ram_GB(v)))
        }
        "Фронтальная камера (Мп)" => {
            int_mp_value(context, &value).and_then(|v| Some(IntCharacteristic::FrontCamera_MP(v)))
        }
        "Разрешение видеосъемки (пикс)" => pix_int_value(context, &value)
            .and_then(|v| Some(IntCharacteristic::VideoResolution_Pix(v))),
        "Емкость аккумулятора (мА*ч)" => int_ma_h_value(context, &value)
            .and_then(|v| Some(IntCharacteristic::BatteryCapacity_mA_h(v))),
        "Кол-во SIM-карт" => {
            int_value(context, &value).and_then(|v| Some(IntCharacteristic::AmountOfSimCards(v)))
        }
        "Частота кадров видеосъемки" => {
            int_fps_value(context, &value).and_then(|v| Some(IntCharacteristic::Fps(v)))
        }
        "Плотность пикселей (PPI)" => {
            int_value(context, &value).and_then(|v| Some(IntCharacteristic::PPI(v)))
        }
        "Максимальный объем карты памяти" => {
            int_memory_value(context, &value)
                .and_then(|v| Some(IntCharacteristic::MaxMemoryCardSize_GB(v)))
        }
        "Яркость (кд/м²)" => int_nit_value(context, &value)
            .and_then(|v| Some(IntCharacteristic::Brightness_cd_m2(v))),
        "Частота обновления" => int_hz_value(context, &value)
            .and_then(|v| Some(IntCharacteristic::UpdateFrequency_Hz(v))),
        "Фотокамера (Мп)" => {
            int_mp_value(context, &value).and_then(|v| Some(IntCharacteristic::Camera_mp(v)))
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
            .map(|v| IntCharacteristic::LTEDiapason(v))
            .collect(),
        "Диапазоны GSM" => multiple_int_value(context, value)
            .into_iter()
            .map(|v| IntCharacteristic::GSMDiapason(v))
            .collect(),
        "Диапазоны UMTS" => multiple_int_value(context, value)
            .into_iter()
            .map(|v| IntCharacteristic::UMTSDiapason(v))
            .collect(),
        _ => vec![],
    }
}

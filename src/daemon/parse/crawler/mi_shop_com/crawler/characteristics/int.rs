use crate::daemon::parse::crawler::characteristic_parser::*;
use crate::daemon::parse::crawler::crawler::Crawler;
use crate::daemon::parse::crawler::mi_shop_com::crawler::MiShopComCrawler;
use crate::common::dto::characteristic::int_characteristic::IntCharacteristic;

pub fn extract_int_characteristics(
    crawler: &MiShopComCrawler,
    external_id: &str,
    titles: &Vec<String>,
    values: &Vec<String>,
) -> (Vec<IntCharacteristic>, Vec<usize>) {
    let mut characteristics: Vec<IntCharacteristic> = vec![];
    let mut parsed_indexes = vec![];

    for (title_index, title) in titles.into_iter().enumerate() {
        let value = values.get(title_index).unwrap();
        let context = CharacteristicParsingContext {
            title: &title,
            external_id,
            source: crawler.get_source(),
        };

        match title.as_str() {
            "Диапазоны LTE" => {
                multiple_int_value(&context, value)
                    .into_iter()
                    .for_each(|v| characteristics.push(IntCharacteristic::LTEDiapason(v)));
                parsed_indexes.push(title_index);
            }
            "Диапазоны GSM" => {
                multiple_int_value(&context, value)
                    .into_iter()
                    .for_each(|v| characteristics.push(IntCharacteristic::GSMDiapason(v)));
                parsed_indexes.push(title_index);
            }
            "Диапазоны UMTS" => {
                multiple_int_value(&context, value)
                    .into_iter()
                    .for_each(|v| characteristics.push(IntCharacteristic::UMTSDiapason(v)));
                parsed_indexes.push(title_index);
            }
            _ => (),
        }
    }
    for (title_index, title) in titles.into_iter().enumerate() {
        let value = values.get(title_index).unwrap();
        let context = CharacteristicParsingContext {
            title: &title,
            external_id,
            source: crawler.get_source(),
        };

        let characteristic: Option<IntCharacteristic> = match title.as_str() {
            "Количество ядер процессора" => int_value(&context, &value)
                .and_then(|v| Some(IntCharacteristic::NumberOfProcessorCores(v))),
            "Гарантия (мес)" => int_guarantee_value(&context, &value)
                .and_then(|v| Some(IntCharacteristic::Warranty_month(v))),
            "Встроенная память (ГБ)" => int_value(&context, &value)
                .and_then(|v| Some(IntCharacteristic::BuiltInMemory_GB(v))),
            "Оперативная память (ГБ)" => {
                int_value(&context, &value).and_then(|v| Some(IntCharacteristic::Ram_GB(v)))
            }
            "Фронтальная камера (Мп)" => int_mp_value(&context, &value)
                .and_then(|v| Some(IntCharacteristic::FrontCamera_MP(v))),
            "Разрешение видеосъемки (пикс)" => {
                pix_int_value(&context, &value)
                    .and_then(|v| Some(IntCharacteristic::VideoResolution_Pix(v)))
            }
            "Емкость аккумулятора (мА*ч)" => int_ma_h_value(&context, &value)
                .and_then(|v| Some(IntCharacteristic::BatteryCapacity_mA_h(v))),
            "Кол-во SIM-карт" => int_value(&context, &value)
                .and_then(|v| Some(IntCharacteristic::AmountOfSimCards(v))),
            "Частота кадров видеосъемки" => {
                int_fps_value(&context, &value).and_then(|v| Some(IntCharacteristic::Fps(v)))
            }
            "Плотность пикселей (PPI)" => {
                int_value(&context, &value).and_then(|v| Some(IntCharacteristic::PPI(v)))
            }
            "Максимальный объем карты памяти" => {
                int_memory_value(&context, &value)
                    .and_then(|v| Some(IntCharacteristic::MaxMemoryCardSize_GB(v)))
            }
            "Яркость (кд/м²)" => int_nit_value(&context, &value)
                .and_then(|v| Some(IntCharacteristic::Brightness_cd_m2(v))),
            "Частота обновления" => int_hz_value(&context, &value)
                .and_then(|v| Some(IntCharacteristic::UpdateFrequency_Hz(v))),
            "Фотокамера (Мп)" => {
                int_mp_value(&context, &value).and_then(|v| Some(IntCharacteristic::Camera_mp(v)))
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

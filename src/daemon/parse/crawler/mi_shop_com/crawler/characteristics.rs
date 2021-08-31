use regex::Regex;
use scraper::{ElementRef, Html, Selector};

use crate::common::dto::characteristic::enum_characteristic::{EnumCharacteristic, Technology};
use crate::common::dto::characteristic::float_characteristic::FloatCharacteristic;
use crate::common::dto::characteristic::int_characteristic::IntCharacteristic;
use crate::common::dto::characteristic::string_characteristic::StringCharacteristic;
use crate::common::dto::characteristic::TypedCharacteristic;
use crate::daemon::parse::crawler::characteristic_parser::*;
use crate::daemon::parse::crawler::crawler::Crawler;
use crate::daemon::parse::crawler::mi_shop_com::crawler::media_format::multiple_string_media_format_value;
use crate::daemon::parse::crawler::mi_shop_com::crawler::MiShopComCrawler;
use crate::daemon::service::html_cleaner::inner_text;

lazy_static! {
    static ref NO_DESCRIPTION_RE: Regex = Regex::new(r"(?ms)[A-Za-z./ 0-9\-+–]{2,}").unwrap();
}

pub fn extract_characteristics(
    crawler: &MiShopComCrawler,
    document: &Html,
    external_id: &str,
) -> Vec<TypedCharacteristic> {
    let characteristic_title_selector =
        Selector::parse(".detail__table tr td.detail__table-one").unwrap();
    let characteristic_value_selector =
        Selector::parse(".detail__table tr td.detail__table-two").unwrap();
    let characteristic_title_nodes = document.select(&characteristic_title_selector);
    let characteristic_value_nodes = document.select(&characteristic_value_selector);

    let mut characteristics: Vec<TypedCharacteristic> = vec![];
    let mut titles: Vec<String> = characteristic_title_nodes
        .into_iter()
        .collect::<Vec<ElementRef>>()
        .into_iter()
        .map(|title| inner_text(&title.inner_html()).replace(":", ""))
        .collect();

    let mut values: Vec<String> = characteristic_value_nodes
        .into_iter()
        .collect::<Vec<ElementRef>>()
        .into_iter()
        .map(|title| inner_text(&title.inner_html()))
        .collect();

    let (int_characteristics, mut parsed_indexes) =
        extract_int_characteristics(crawler, external_id, &titles, &values);
    for int_char in int_characteristics {
        characteristics.push(TypedCharacteristic::Int(int_char));
    }
    remove_parsed_indexes(&mut titles, &mut values, &mut parsed_indexes);

    let (float_characteristics, mut parsed_indexes) =
        extract_float_characteristics(crawler, external_id, &titles, &values);
    for float_char in float_characteristics {
        characteristics.push(TypedCharacteristic::Float(float_char));
    }
    remove_parsed_indexes(&mut titles, &mut values, &mut parsed_indexes);

    let (string_characteristics, mut parsed_indexes) =
        extract_string_characteristics(&titles, &values);
    for string_char in string_characteristics {
        characteristics.push(TypedCharacteristic::String(string_char));
    }
    remove_parsed_indexes(&mut titles, &mut values, &mut parsed_indexes);

    let (enum_characteristics, mut parsed_indexes) =
        extract_enum_characteristics(crawler, external_id, &titles, &values);
    for string_char in enum_characteristics {
        characteristics.push(TypedCharacteristic::Enum(string_char));
    }
    remove_parsed_indexes(&mut titles, &mut values, &mut parsed_indexes);

    let (technology_characteristics, mut parsed_indexes) =
        extract_technology_characteristics(crawler, external_id, &titles, &values);
    for string_char in technology_characteristics {
        characteristics.push(TypedCharacteristic::Enum(string_char));
    }
    remove_parsed_indexes(&mut titles, &mut values, &mut parsed_indexes);

    let mut parsed_indexes = skip_unneeded_characteristics(&titles);
    remove_parsed_indexes(&mut titles, &mut values, &mut parsed_indexes);

    for (title_index, title) in titles.into_iter().enumerate() {
        let value = values.get(title_index).unwrap();
        sentry::capture_message(
            format!(
                "Unknown characteristic ({title}) with value ({value}) for [{external_id}]",
                title = title,
                value = value,
                external_id = external_id,
            )
            .as_str(),
            sentry::Level::Warning,
        );
    }
    characteristics
}

fn remove_parsed_indexes(
    titles: &mut Vec<String>,
    values: &mut Vec<String>,
    parsed_indexes: &mut Vec<usize>,
) {
    parsed_indexes.sort_by(|a, b| b.cmp(a));
    for index in parsed_indexes {
        titles.remove(*index);
        values.remove(*index);
    }
}

fn skip_unneeded_characteristics(titles: &Vec<String>) -> Vec<usize> {
    let mut parsed_indexes = vec![];
    for (title_index, title) in titles.into_iter().enumerate() {
        let skip: bool = match title.as_str() {
            "Видеозапись" => true,
            "Сенсорный дисплей" => true,
            "Примечание" => true,
            "Видеоплеер" => true,
            "Аудиоплеер" => true,
            _ => false,
        };

        if skip {
            parsed_indexes.push(title_index);
        }
    }

    parsed_indexes
}

fn extract_technology_characteristics(
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
        let characteristic: Option<Technology> =
            match title.as_str() {
                "NFC" => {
                    parsed_indexes.push(title_index);
                    bool_value(&context, &value).and_then(|v| {
                        if v {
                            Some(Technology::NFC)
                        } else {
                            None
                        }
                    })
                }
                "Автофокус" => {
                    parsed_indexes.push(title_index);
                    bool_value(&context, &value).and_then(|v| {
                        if v {
                            Some(Technology::Autofocus)
                        } else {
                            None
                        }
                    })
                }
                "Быстрая зарядка" => {
                    parsed_indexes.push(title_index);
                    bool_value(&context, &value).and_then(|v| {
                        if v {
                            Some(Technology::FastCharging)
                        } else {
                            None
                        }
                    })
                }
                "ИК-порт" => {
                    parsed_indexes.push(title_index);
                    bool_value(&context, &value).and_then(|v| {
                        if v {
                            Some(Technology::InfraredPort)
                        } else {
                            None
                        }
                    })
                }
                "Беспроводная зарядка" => {
                    parsed_indexes.push(title_index);
                    bool_value(&context, &value).and_then(|v| {
                        if v {
                            Some(Technology::WirelessCharger)
                        } else {
                            None
                        }
                    })
                }
                _ => None,
            };

        if let Some(characteristic) = characteristic {
            characteristics.push(EnumCharacteristic::TechnologySupport(characteristic));
        }
    }

    (characteristics, parsed_indexes)
}

fn extract_string_characteristics(
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

fn extract_enum_characteristics(
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

fn extract_float_characteristics(
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

fn extract_int_characteristics(
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

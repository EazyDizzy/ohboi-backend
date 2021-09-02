use crate::common::dto::characteristic::enum_characteristic::{EnumCharacteristic, Technology};
use crate::daemon::parse::crawler::characteristic_parser::*;
use crate::daemon::parse::crawler::crawler::Crawler;
use crate::daemon::parse::crawler::mi_shop_com::crawler::MiShopComCrawler;

pub fn extract_technology_characteristics(
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

pub fn extract_technology_characteristic(title: &str, value: &str, context: CharacteristicParsingContext) -> Option<EnumCharacteristic> {
    let characteristic = match title {
        "NFC" => {
            bool_value(&context, &value).and_then(|v| if v { Some(Technology::NFC) } else { None })
        }
        "Автофокус" => {
            bool_value(&context, &value).and_then(|v| {
                if v {
                    Some(Technology::Autofocus)
                } else {
                    None
                }
            })
        }
        "Быстрая зарядка" => {
            bool_value(&context, &value).and_then(|v| {
                if v {
                    Some(Technology::FastCharging)
                } else {
                    None
                }
            })
        }
        "ИК-порт" => {
            bool_value(&context, &value).and_then(|v| {
                if v {
                    Some(Technology::InfraredPort)
                } else {
                    None
                }
            })
        }
        "Беспроводная зарядка" => {
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

    characteristic.and_then(|v| Some(EnumCharacteristic::TechnologySupport(v)))
}

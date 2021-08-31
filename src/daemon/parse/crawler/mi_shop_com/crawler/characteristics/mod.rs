use scraper::{ElementRef, Html, Selector};

use crate::common::dto::characteristic::TypedCharacteristic;
use crate::daemon::parse::crawler::mi_shop_com::crawler::characteristics::enums::extract_enum_characteristics;
use crate::daemon::parse::crawler::mi_shop_com::crawler::characteristics::float::extract_float_characteristics;
use crate::daemon::parse::crawler::mi_shop_com::crawler::characteristics::int::extract_int_characteristics;
use crate::daemon::parse::crawler::mi_shop_com::crawler::characteristics::strings::extract_string_characteristics;
use crate::daemon::parse::crawler::mi_shop_com::crawler::characteristics::technology::extract_technology_characteristics;
use crate::daemon::parse::crawler::mi_shop_com::crawler::MiShopComCrawler;
use crate::daemon::service::html_cleaner::inner_text;

mod enums;
mod float;
mod int;
mod strings;
mod technology;

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

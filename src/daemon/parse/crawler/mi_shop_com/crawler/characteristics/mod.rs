use scraper::{ElementRef, Html, Selector};

use lib::dto::characteristic::TypedCharacteristic;
use crate::daemon::parse::crawler::characteristic_parser::{
    combine_titles_and_values, parse_and_take, parse_and_take_multiple,
};
use crate::daemon::parse::crawler::mi_shop_com::crawler::characteristics::enums::extract_enum_characteristic;
use crate::daemon::parse::crawler::mi_shop_com::crawler::characteristics::float::extract_float_characteristic;
use crate::daemon::parse::crawler::mi_shop_com::crawler::characteristics::int::extract_int_characteristic;
use crate::daemon::parse::crawler::mi_shop_com::crawler::characteristics::skip::skip_unneeded_characteristics;
use crate::daemon::parse::crawler::mi_shop_com::crawler::characteristics::strings::extract_string_characteristic;
use crate::daemon::parse::crawler::mi_shop_com::crawler::characteristics::technology::{
    extract_technology_characteristic,
};
use crate::daemon::parse::crawler::mi_shop_com::crawler::MiShopComCrawler;
use crate::daemon::service::html_cleaner::inner_text;

mod enums;
mod float;
mod int;
mod skip;
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

    let mut parsed_characteristics: Vec<TypedCharacteristic> = vec![];
    let titles: Vec<String> = characteristic_title_nodes
        .into_iter()
        .collect::<Vec<ElementRef>>()
        .into_iter()
        .map(|title| inner_text(&title.inner_html()).replace(":", ""))
        .collect();

    let values: Vec<String> = characteristic_value_nodes
        .into_iter()
        .collect::<Vec<ElementRef>>()
        .into_iter()
        .map(|title| inner_text(&title.inner_html()))
        .collect();

    let mut characteristics = combine_titles_and_values(titles, values);

    let string_chars = parse_and_take(
        &mut characteristics,
        crawler,
        external_id,
        extract_string_characteristic,
    );
    for string_char in string_chars {
        parsed_characteristics.push(TypedCharacteristic::String(string_char));
    }

    let float_chars = parse_and_take(
        &mut characteristics,
        crawler,
        external_id,
        extract_float_characteristic,
    );
    for float_char in float_chars {
        parsed_characteristics.push(TypedCharacteristic::Float(float_char));
    }

    let int_chars = parse_and_take_multiple(
        &mut characteristics,
        crawler,
        external_id,
        extract_int_characteristic,
    );
    for int_char in int_chars {
        parsed_characteristics.push(TypedCharacteristic::Int(int_char));
    }

    let enum_chars = parse_and_take_multiple(
        &mut characteristics,
        crawler,
        external_id,
        extract_enum_characteristic,
    );
    for enum_char in enum_chars {
        parsed_characteristics.push(TypedCharacteristic::Enum(enum_char));
    }

    let technology_characteristics = parse_and_take(
        &mut characteristics,
        crawler,
        external_id,
        extract_technology_characteristic,
    );
    for technology_char in technology_characteristics {
        parsed_characteristics.push(TypedCharacteristic::Enum(technology_char));
    }

    parse_and_take::<bool>(
        &mut characteristics,
        crawler,
        external_id,
        skip_unneeded_characteristics,
    );

    for (title, value) in characteristics.into_iter() {
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

    parsed_characteristics
}

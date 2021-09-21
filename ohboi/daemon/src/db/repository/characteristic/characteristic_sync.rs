use strum::VariantNames;

use lib::db::entity::characteristic::Characteristic;
use lib::diesel::prelude::*;
use lib::dto::characteristic::enum_characteristic::{
    AudioJack, BatteryType, ChargingConnectorType, Country, DisplayType, EnumCharacteristic,
    InternetConnectionTechnology, Material, MediaFormat, MemoryCardSlot, SatelliteNavigation,
    SimCard, Technology, WifiStandard,
};
use lib::error_reporting::ReportingContext;
use lib::schema::category_characteristic;
use lib::util::all_characteristics::{
    get_enum_characteristics, get_float_characteristics, get_int_characteristics,
    get_string_characteristics,
};
use lib::{db, error_reporting};

use crate::db::entity::category::CategorySlug;
use crate::db::entity::characteristic::category_characteristic::{
    CategoryCharacteristic, NewCategoryCharacteristic,
};
use crate::db::repository::category::get_category;
use crate::db::repository::characteristic::{characteristic, product_characteristic_enum_value};
use crate::db::Executor;

// TODO update if sth changed
// TODO delete removed
pub fn sync_characteristic_enum() {
    sync_float_chars();
    sync_int_chars();
    sync_string_chars();
    sync_enum_chars();
}

fn sync_float_chars() {
    for item in get_float_characteristics() {
        let created_char = characteristic::create_if_not_exists(
            item.id,
            item.slug,
            item.visualisation_type,
            item.value_type,
        );
        if let Some(c) = created_char {
            connect_char_to_category(c, CategorySlug::Smartphone)
        }
    }
}

fn sync_int_chars() {
    for item in get_int_characteristics() {
        let created_char = characteristic::create_if_not_exists(
            item.id,
            item.slug,
            item.visualisation_type,
            item.value_type,
        );
        if let Some(c) = created_char {
            connect_char_to_category(c, CategorySlug::Smartphone)
        }
    }
}

fn sync_string_chars() {
    for item in get_string_characteristics() {
        let created_char = characteristic::create_if_not_exists(
            item.id,
            item.slug,
            item.visualisation_type,
            item.value_type,
        );
        if let Some(c) = created_char {
            connect_char_to_category(c, CategorySlug::Smartphone)
        }
    }
}

fn sync_enum_chars() {
    for item in get_enum_characteristics() {
        let created_char = characteristic::create_if_not_exists(
            item.id,
            item.slug,
            item.visualisation_type,
            item.value_type,
        );
        if let Some(c) = created_char {
            connect_char_to_category(c, CategorySlug::Smartphone)
        }
    }

    sync_enum_char_values();
}

fn sync_enum_char_values() {
    sync_one_enum_char_values(EnumCharacteristic::ChargingConnectorType(
        ChargingConnectorType::USBTypeC,
    ));
    sync_one_enum_char_values(EnumCharacteristic::BatteryType(BatteryType::LithiumIon));
    sync_one_enum_char_values(EnumCharacteristic::SimCard(SimCard::Mini));
    sync_one_enum_char_values(EnumCharacteristic::Material(Material::Plastic));
    sync_one_enum_char_values(EnumCharacteristic::DisplayType(DisplayType::Oled));
    sync_one_enum_char_values(EnumCharacteristic::InternetConnectionTechnology(
        InternetConnectionTechnology::_4G,
    ));
    sync_one_enum_char_values(EnumCharacteristic::SatelliteNavigation(
        SatelliteNavigation::Galileo,
    ));
    sync_one_enum_char_values(EnumCharacteristic::WifiStandard(WifiStandard::_5));
    sync_one_enum_char_values(EnumCharacteristic::AudioJack(AudioJack::USBTypeC));
    sync_one_enum_char_values(EnumCharacteristic::TechnologySupport(
        Technology::FastCharging,
    ));
    sync_one_enum_char_values(EnumCharacteristic::ProducingCountry(Country::China));
    sync_one_enum_char_values(EnumCharacteristic::MemoryCardSlot(MemoryCardSlot::Separate));
    sync_one_enum_char_values(EnumCharacteristic::SupportedMediaFormat(MediaFormat::H264));
}

/// This code was moved to separate function just to force compiler to fail when new variant was added
/// Don't forget to add new variant to sync_enum_char_values when adding below
fn sync_one_enum_char_values(char: EnumCharacteristic) {
    let values = match char {
        EnumCharacteristic::ChargingConnectorType(_) => ChargingConnectorType::VARIANTS,
        EnumCharacteristic::BatteryType(_) => BatteryType::VARIANTS,
        EnumCharacteristic::SimCard(_) => SimCard::VARIANTS,
        EnumCharacteristic::Material(_) => Material::VARIANTS,
        EnumCharacteristic::DisplayType(_) => DisplayType::VARIANTS,
        EnumCharacteristic::InternetConnectionTechnology(_) => {
            InternetConnectionTechnology::VARIANTS
        }
        EnumCharacteristic::SatelliteNavigation(_) => SatelliteNavigation::VARIANTS,
        EnumCharacteristic::WifiStandard(_) => WifiStandard::VARIANTS,
        EnumCharacteristic::AudioJack(_) => AudioJack::VARIANTS,
        EnumCharacteristic::TechnologySupport(_) => Technology::VARIANTS,
        EnumCharacteristic::ProducingCountry(_) => Country::VARIANTS,
        EnumCharacteristic::MemoryCardSlot(_) => MemoryCardSlot::VARIANTS,
        EnumCharacteristic::SupportedMediaFormat(_) => MediaFormat::VARIANTS,
    };

    for value in values {
        product_characteristic_enum_value::create_if_not_exists(
            [char.name().as_str(), ".", value].concat(),
        );
    }
}

fn connect_char_to_category(char: Characteristic, category: CategorySlug) {
    let connection = &db::establish_connection();
    let category_id = get_category(category).id;

    let new_category_char = NewCategoryCharacteristic {
        characteristic_id: char.id,
        category_id,
    };

    let insert_result: Result<CategoryCharacteristic, diesel::result::Error> =
        diesel::insert_into(category_characteristic::table)
            .values(&new_category_char)
            .get_result(connection);

    match insert_result {
        Ok(_) => {
            log::info!(
                "Characteristic {} was successfully matched to {} category",
                char.slug,
                category
            );
        }
        Err(e) => {
            error_reporting::fatal(
                format!(
                    "Characteristic {} can't be matched to {} category. {:?}",
                    char.slug, category, e
                )
                .as_str(),
                &ReportingContext {
                    executor: &Executor::Characteristic,
                    action: "save_characteristic_value",
                },
            );
        }
    }
}

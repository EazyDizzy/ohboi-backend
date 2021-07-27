use diesel::result::{DatabaseErrorKind, Error};
use strum::IntoEnumIterator;
use strum::VariantNames;

use crate::common::db;
use crate::diesel::prelude::*;
use crate::my_enum::{CharacteristicValueType, CharacteristicVisualisationType};
use crate::parse::db::entity::category::CategorySlug;
use crate::parse::db::entity::characteristic::category_characteristic::{
    CategoryCharacteristic, NewCategoryCharacteristic,
};
use crate::parse::db::entity::characteristic::characteristic::{Characteristic, NewCharacteristic};
use crate::parse::db::entity::characteristic::product_characteristic_enum_value::{
    NewProductCharacteristicEnumValue, ProductCharacteristicEnumValue,
};
use crate::parse::db::repository::category::get_category;
use crate::parse::dto::characteristic::enum_characteristic::*;
use crate::parse::dto::characteristic::float_characteristic::FloatCharacteristic;
use crate::parse::dto::characteristic::int_characteristic::IntCharacteristic;
use crate::parse::dto::characteristic::string_characteristic::StringCharacteristic;
use crate::schema::category_characteristic;
use crate::schema::characteristic;
use crate::schema::product_characteristic_enum_value;

// TODO update if sth changed
// TODO delete removed
pub fn sync_characteristic_enum() -> () {
    sync_float_chars();
    sync_int_chars();
    sync_string_chars();
    sync_enum_chars();
}

fn sync_float_chars() {
    let connection = &db::establish_connection();

    for item in FloatCharacteristic::iter() {
        let value_type = CharacteristicValueType::Float;
        let visualisation_type = get_float_char_vis_type(item);

        let new_char = NewCharacteristic {
            slug: item.name(),
            enabled: true,
            visualisation_type,
            value_type,
        };

        let insert_result: Result<Characteristic, diesel::result::Error> =
            diesel::insert_into(characteristic::table)
                .values(&new_char)
                .get_result(connection);

        match insert_result {
            Ok(new_char) => {
                log::info!("Float {} characteristic was created", new_char.slug);
                connect_char_to_category(new_char, CategorySlug::Smartphone);
            }
            Err(e) => {
                if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) = e {
                    log::info!("Float {} characteristic already exists", new_char.slug);
                } else {
                    sentry::capture_message(
                        format!(
                            "Float {} characteristic has an error: {:?}",
                            new_char.slug, e
                        )
                        .as_str(),
                        sentry::Level::Warning,
                    );
                }
            }
        }
    }
}

fn get_float_char_vis_type(char: FloatCharacteristic) -> CharacteristicVisualisationType {
    use FloatCharacteristic::*;

    match char {
        Width_mm(_) => CharacteristicVisualisationType::Range,
        Height_mm(_) => CharacteristicVisualisationType::Range,
        Thickness_mm(_) => CharacteristicVisualisationType::Range,
        ScreenDiagonal(_) => CharacteristicVisualisationType::MultiSelector,
        BluetoothVersion(_) => CharacteristicVisualisationType::MultiSelector,
        CPUFrequency_Ghz(_) => CharacteristicVisualisationType::MultiSelector,
        Weight_gr(_) => CharacteristicVisualisationType::Range,
        MIUIVersion(_) => CharacteristicVisualisationType::MultiSelector,
        AndroidVersion(_) => CharacteristicVisualisationType::MultiSelector,
        Aperture(_) => CharacteristicVisualisationType::MultiSelector,
    }
}

fn sync_int_chars() {
    let connection = &db::establish_connection();

    for item in IntCharacteristic::iter() {
        let value_type = CharacteristicValueType::Int;
        let visualisation_type = get_int_char_vis_type(item);

        let new_char = NewCharacteristic {
            slug: item.name(),
            enabled: true,
            visualisation_type,
            value_type,
        };

        let insert_result: Result<Characteristic, diesel::result::Error> =
            diesel::insert_into(characteristic::table)
                .values(&new_char)
                .get_result(connection);

        match insert_result {
            Ok(new_char) => {
                log::info!("Int {} characteristic was created", new_char.slug);
                connect_char_to_category(new_char, CategorySlug::Smartphone);
            }
            Err(e) => {
                if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) = e {
                    log::info!("Int {} characteristic already exists", new_char.slug);
                } else {
                    sentry::capture_message(
                        format!("Int {} characteristic has an error: {:?}", new_char.slug, e)
                            .as_str(),
                        sentry::Level::Warning,
                    );
                }
            }
        }
    }
}

fn get_int_char_vis_type(char: IntCharacteristic) -> CharacteristicVisualisationType {
    use IntCharacteristic::*;

    match char {
        BatteryCapacity_mA_h(_) => CharacteristicVisualisationType::MultiSelector,
        NumberOfProcessorCores(_) => CharacteristicVisualisationType::MultiSelector,
        BuiltInMemory_GB(_) => CharacteristicVisualisationType::MultiSelector,
        Ram_GB(_) => CharacteristicVisualisationType::MultiSelector,
        FrontCamera_MP(_) => CharacteristicVisualisationType::MultiSelector,
        VideoResolution_Pix(_) => CharacteristicVisualisationType::MultiSelector,
        AmountOfSimCards(_) => CharacteristicVisualisationType::MultiSelector,
        PPI(_) => CharacteristicVisualisationType::MultiSelector,
        Fps(_) => CharacteristicVisualisationType::MultiSelector,
        Brightness_cd_m2(_) => CharacteristicVisualisationType::MultiSelector,
        UpdateFrequency_Hz(_) => CharacteristicVisualisationType::MultiSelector,
        Camera_mp(_) => CharacteristicVisualisationType::MultiSelector,
        LTEDiapason(_) => CharacteristicVisualisationType::MultiSelector,
        GSMDiapason(_) => CharacteristicVisualisationType::MultiSelector,
        UMTSDiapason(_) => CharacteristicVisualisationType::MultiSelector,
        Warranty_month(_) => CharacteristicVisualisationType::MultiSelector,
        MaxMemoryCardSize_GB(_) => CharacteristicVisualisationType::MultiSelector,
    }
}
fn sync_string_chars() {
    let connection = &db::establish_connection();

    for item in StringCharacteristic::iter() {
        let value_type = CharacteristicValueType::String;
        let visualisation_type = CharacteristicVisualisationType::MultiSelector;

        let new_char = NewCharacteristic {
            slug: item.name(),
            enabled: true,
            visualisation_type,
            value_type,
        };

        let insert_result: Result<Characteristic, diesel::result::Error> =
            diesel::insert_into(characteristic::table)
                .values(&new_char)
                .get_result(connection);

        match insert_result {
            Ok(new_char) => {
                log::info!("String {} characteristic was created", new_char.slug);
                connect_char_to_category(new_char, CategorySlug::Smartphone);
            }
            Err(e) => {
                if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) = e {
                    log::info!("String {} characteristic already exists", new_char.slug);
                } else {
                    sentry::capture_message(
                        format!(
                            "String {} characteristic has an error: {:?}",
                            new_char.slug, e
                        )
                        .as_str(),
                        sentry::Level::Warning,
                    );
                }
            }
        }
    }
}

fn sync_enum_chars() {
    let connection = &db::establish_connection();

    for item in EnumCharacteristic::VARIANTS {
        let value_type = CharacteristicValueType::Enum;
        let visualisation_type = CharacteristicVisualisationType::MultiSelector;

        let new_char = NewCharacteristic {
            slug: item.to_string(),
            enabled: true,
            visualisation_type,
            value_type,
        };

        let insert_result: Result<Characteristic, diesel::result::Error> =
            diesel::insert_into(characteristic::table)
                .values(&new_char)
                .get_result(connection);

        match insert_result {
            Ok(new_char) => {
                log::info!("Enum {} characteristic was created", new_char.slug);
                connect_char_to_category(new_char, CategorySlug::Smartphone);
            }
            Err(e) => {
                if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) = e {
                    log::info!("Enum {} characteristic already exists", new_char.slug);
                } else {
                    sentry::capture_message(
                        format!(
                            "Enum {} characteristic has an error: {:?}",
                            new_char.slug, e
                        )
                        .as_str(),
                        sentry::Level::Warning,
                    );
                }
            }
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
    let connection = &db::establish_connection();

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
        let new_enum_char_value = NewProductCharacteristicEnumValue {
            value: [char.name().as_str(), ".", value].concat(),
        };

        let insert_result: Result<ProductCharacteristicEnumValue, diesel::result::Error> =
            diesel::insert_into(product_characteristic_enum_value::table)
                .values(&new_enum_char_value)
                .get_result(connection);

        match insert_result {
            Ok(_) => {
                log::info!(
                    "Enum characteristic value {} was created",
                    new_enum_char_value.value
                );
            }
            Err(e) => {
                if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) = e {
                    log::info!(
                        "Enum characteristic value {} already exists",
                        new_enum_char_value.value
                    );
                } else {
                    sentry::capture_message(
                        format!(
                            "Enum characteristic value {} has an error: {:?}",
                            new_enum_char_value.value, e
                        )
                        .as_str(),
                        sentry::Level::Warning,
                    );
                }
            }
        }
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
            sentry::capture_message(
                format!(
                    "Characteristic {} can't be matched to {} category. {:?}",
                    char.slug, category, e
                )
                .as_str(),
                sentry::Level::Warning,
            );
        }
    }
}

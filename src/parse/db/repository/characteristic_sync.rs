use diesel::result::{DatabaseErrorKind, Error};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::common::db;
use crate::diesel::prelude::*;
use crate::my_enum::{CharacteristicValueType, CharacteristicVisualisationType};
use crate::parse::db::entity::characteristic::characteristic::{Characteristic, NewCharacteristic};
use crate::parse::dto::characteristic::float_characteristic::FloatCharacteristic;
use crate::parse::dto::characteristic::int_characteristic::IntCharacteristic;
use crate::parse::dto::characteristic::string_characteristic::StringCharacteristic;
use crate::parse::dto::parsed_product::TypedCharacteristic;
use crate::schema::characteristic;

// TODO update if sth changed
// TODO delete removed
pub fn sync_characteristic_enum() -> () {
    sync_float_chars();
    sync_int_chars();
    sync_string_chars();

    match_characteristics_to_categories();
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
            Ok(_) => {
                log::info!("Float {} characteristic was created", new_char.slug);
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
            Ok(_) => {
                log::info!("Int {} characteristic was created", new_char.slug);
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
            Ok(_) => {
                log::info!("String {} characteristic was created", new_char.slug);
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

fn match_characteristics_to_categories() {
    todo!()
}

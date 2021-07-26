use diesel::result::{DatabaseErrorKind, Error};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::common::db;
use crate::diesel::prelude::*;
use crate::my_enum::{CharacteristicValueType, CharacteristicVisualisationType};
use crate::parse::db::entity::characteristic::characteristic::{Characteristic, NewCharacteristic};
use crate::parse::dto::characteristic::float_characteristic::FloatCharacteristic;
use crate::parse::dto::characteristic::int_characteristic::IntCharacteristic;
use crate::parse::dto::parsed_product::TypedCharacteristic;
use crate::schema::characteristic;

// TODO update if sth changed
pub fn sync_characteristic_enum() -> () {
    sync_float_chars();
    sync_int_chars();
    sync_string_chars();

    match_characteristics_to_categories();
}

fn sync_float_chars() {
    let connection = &db::establish_connection();

    for item in FloatCharacteristic::iter() {
        let value_type = CharacteristicValueType::FLOAT;
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
                        format!("Float {} characteristic has an error: {:?}", new_char.slug, e).as_str(),
                        sentry::Level::Warning,
                    );
                }
            }
        }
    }
}

fn get_float_char_vis_type(char: FloatCharacteristic) -> CharacteristicVisualisationType {
    match char {
        FloatCharacteristic::Width_mm(_) => CharacteristicVisualisationType::Range,
        FloatCharacteristic::Height_mm(_) => CharacteristicVisualisationType::Range,
        FloatCharacteristic::Thickness_mm(_) => CharacteristicVisualisationType::Range,
        FloatCharacteristic::ScreenDiagonal(_) => CharacteristicVisualisationType::MultiSelector,
        FloatCharacteristic::BluetoothVersion(_) => CharacteristicVisualisationType::MultiSelector,
        FloatCharacteristic::CPUFrequency_Ghz(_) => CharacteristicVisualisationType::MultiSelector,
        FloatCharacteristic::Weight_gr(_) => CharacteristicVisualisationType::Range,
        FloatCharacteristic::MIUIVersion(_) => CharacteristicVisualisationType::MultiSelector,
        FloatCharacteristic::AndroidVersion(_) => CharacteristicVisualisationType::MultiSelector,
        FloatCharacteristic::Aperture(_) => CharacteristicVisualisationType::MultiSelector,
    }
}

fn sync_int_chars() {
    let connection = &db::establish_connection();

    for item in IntCharacteristic::iter() {
        let value_type = CharacteristicValueType::INT;
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
                        format!("Int {} characteristic has an error: {:?}", new_char.slug, e).as_str(),
                        sentry::Level::Warning,
                    );
                }
            }
        }
    }
}

fn get_int_char_vis_type(char: IntCharacteristic) -> CharacteristicVisualisationType {
    match char {
        IntCharacteristic::BatteryCapacity_mA_h(_) => {
            CharacteristicVisualisationType::MultiSelector
        }
        IntCharacteristic::NumberOfProcessorCores(_) => {
            CharacteristicVisualisationType::MultiSelector
        }
        IntCharacteristic::BuiltInMemory_GB(_) => CharacteristicVisualisationType::MultiSelector,
        IntCharacteristic::Ram_GB(_) => CharacteristicVisualisationType::MultiSelector,
        IntCharacteristic::FrontCamera_MP(_) => CharacteristicVisualisationType::MultiSelector,
        IntCharacteristic::VideoResolution_Pix(_) => CharacteristicVisualisationType::MultiSelector,
        IntCharacteristic::AmountOfSimCards(_) => CharacteristicVisualisationType::MultiSelector,
        IntCharacteristic::PPI(_) => CharacteristicVisualisationType::MultiSelector,
        IntCharacteristic::Fps(_) => CharacteristicVisualisationType::MultiSelector,
        IntCharacteristic::Brightness_cd_m2(_) => CharacteristicVisualisationType::MultiSelector,
        IntCharacteristic::UpdateFrequency_Hz(_) => CharacteristicVisualisationType::MultiSelector,
        IntCharacteristic::Camera_mp(_) => CharacteristicVisualisationType::MultiSelector,
        IntCharacteristic::LTEDiapason(_) => CharacteristicVisualisationType::MultiSelector,
        IntCharacteristic::GSMDiapason(_) => CharacteristicVisualisationType::MultiSelector,
        IntCharacteristic::UMTSDiapason(_) => CharacteristicVisualisationType::MultiSelector,
        IntCharacteristic::Warranty_month(_) => CharacteristicVisualisationType::MultiSelector,
        IntCharacteristic::MaxMemoryCardSize_GB(_) => {
            CharacteristicVisualisationType::MultiSelector
        }
    }
}
fn sync_string_chars() {
    todo!()
}

fn match_characteristics_to_categories() {
    todo!()
}

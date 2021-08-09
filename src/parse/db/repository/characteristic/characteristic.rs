use diesel::result::{DatabaseErrorKind, Error};

use crate::common::db;
use crate::diesel::prelude::*;
use crate::my_enum::{CharacteristicValueType, CharacteristicVisualisationType};
use crate::parse::db::entity::characteristic::characteristic::NewCharacteristic;
use crate::schema::characteristic;
use crate::common::db::entity::characteristic::Characteristic;

pub fn create_if_not_exists(
    id: i16,
    slug: String,
    visualisation_type: CharacteristicVisualisationType,
    value_type: CharacteristicValueType,
) -> Option<Characteristic> {
    let connection = &db::establish_connection();

    let new_char = NewCharacteristic {
        id,
        slug,
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
            log::info!(
                "{:?} {} characteristic was created",
                value_type,
                new_char.slug
            );
            Some(new_char)
        }
        Err(e) => {
            if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) = e {
                log::info!(
                    "{:?} {} characteristic already exists",
                    value_type,
                    new_char.slug
                );
                None
            } else {
                sentry::capture_message(
                    format!(
                        "{:?} {} characteristic has an error: {:?}",
                        value_type, new_char.slug, e
                    )
                    .as_str(),
                    sentry::Level::Warning,
                );
                None
            }
        }
    }
}

use lib::db::characteristic::entity::Characteristic;
use lib::diesel::prelude::*;
use lib::diesel::result::{DatabaseErrorKind, Error};
use lib::error_reporting::ReportingContext;
use lib::my_enum::{
    CharacteristicGroupSlug, CharacteristicValueType, CharacteristicVisualisationType,
};
use lib::schema::characteristic;
use lib::{db, error_reporting};

use crate::db::entity::characteristic::new::NewCharacteristic;
use crate::db::Executor;

#[allow(clippy::too_many_arguments)]
pub fn upsert(
    id: i16,
    slug: String,
    visualisation_type: CharacteristicVisualisationType,
    value_type: CharacteristicValueType,
    sort_key: i16,
    group_slug: CharacteristicGroupSlug,
) -> Option<Characteristic> {
    let connection = &db::establish_connection();

    let new_char = NewCharacteristic {
        id,
        slug: slug.clone(),
        enabled: true,
        visualisation_type,
        value_type,
        sort_key,
        group_slug,
    };

    let upsert_result = diesel::insert_into(characteristic::table)
        .values(&new_char)
        .on_conflict(lib::schema::characteristic::id)
        .do_update()
        .set(&new_char)
        .execute(connection);

    match upsert_result {
        Ok(_) => {
            log::info!(
                "{:?} {} characteristic was created/updated",
                value_type,
                &slug
            );

            Some(Characteristic {
                id,
                slug,
                enabled: true,
                visualisation_type,
                value_type,
                sort_key,
                group_slug,
            })
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
                error_reporting::warning(
                    format!(
                        "{:?} {} characteristic has an error: {:?}",
                        value_type, new_char.slug, e
                    )
                    .as_str(),
                    &ReportingContext {
                        executor: &Executor::Characteristic,
                        action: "save_characteristic",
                    },
                );
                None
            }
        }
    }
}

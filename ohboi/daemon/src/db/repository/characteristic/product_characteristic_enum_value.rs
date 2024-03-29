use lib::{db, error_reporting};
use lib::diesel::prelude::*;
use lib::diesel::result::{DatabaseErrorKind, Error};
use lib::dto::characteristic::enum_characteristic::EnumCharacteristic;
use lib::error_reporting::ReportingContext;

use crate::db::entity::characteristic::product_characteristic_enum_value::{
    NewProductCharacteristicEnumValue, ProductCharacteristicEnumValue,
};
use crate::db::Executor;

// TODO cache
pub fn get_value_by_enum(enm: &EnumCharacteristic) -> ProductCharacteristicEnumValue {
    use lib::schema::product_characteristic_enum_value::dsl::{
        product_characteristic_enum_value, value,
    };
    let v = enm.full_name();
    let connection = &db::establish_connection();

    let filter = value.eq(v);

    let results: Vec<ProductCharacteristicEnumValue> = product_characteristic_enum_value
        .filter(filter)
        .limit(1)
        .load::<ProductCharacteristicEnumValue>(connection)
        .expect("Cannot load product_characteristic_enum_value");

    results.into_iter().next().unwrap()
}

pub fn create_if_not_exists(value: String) {
    use lib::schema::product_characteristic_enum_value;
    let connection = &db::establish_connection();

    let new_enum_char_value = NewProductCharacteristicEnumValue { value };
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
                error_reporting::warning(
                    format!(
                        "Enum characteristic value {} has an error: {:?}",
                        new_enum_char_value.value, e
                    )
                    .as_str(),
                    &ReportingContext {
                        executor: &Executor::Characteristic,
                        action: "save_characteristic_value::enum",
                    },
                );
            }
        }
    }
}

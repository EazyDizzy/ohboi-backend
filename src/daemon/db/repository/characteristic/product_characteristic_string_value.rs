use diesel::result::{DatabaseErrorKind, Error};

use crate::common::db;
use crate::diesel::prelude::*;
use crate::daemon::db::entity::characteristic::product_characteristic_string_value::{
    NewProductCharacteristicStringValue, ProductCharacteristicStringValue,
};

pub fn create_if_not_exists(value: String) -> Option<ProductCharacteristicStringValue> {
    use crate::schema::product_characteristic_string_value;
    let existed_value = get_product_value_by_value(&value);
    if existed_value.is_some() {
        return existed_value;
    }

    let connection = &db::establish_connection();
    let new_char_value = NewProductCharacteristicStringValue {
        value: value.clone(),
    };

    let insert_result: Result<ProductCharacteristicStringValue, diesel::result::Error> =
        diesel::insert_into(product_characteristic_string_value::table)
            .values(&new_char_value)
            .get_result(connection);

    match insert_result {
        Ok(new_char) => Some(new_char),
        Err(e) => {
            if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) = e {
                get_product_value_by_value(&value)
            } else {
                sentry::capture_message(
                    format!(
                        "ProductCharacteristicStringValue with value {} can't be created: {:?}",
                        value, e
                    )
                    .as_str(),
                    sentry::Level::Warning,
                );
                None
            }
        }
    }
}

fn get_product_value_by_value(v: &str) -> Option<ProductCharacteristicStringValue> {
    use crate::schema::product_characteristic_string_value::dsl::{
        product_characteristic_string_value, value,
    };
    let connection = &db::establish_connection();

    let filter = value.eq(v);

    let results: Vec<ProductCharacteristicStringValue> = product_characteristic_string_value
        .filter(filter)
        .limit(1)
        .load::<ProductCharacteristicStringValue>(connection)
        .expect("Cannot load product_characteristic_float_value");

    results.into_iter().next()
}

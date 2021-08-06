use diesel::result::{DatabaseErrorKind, Error};

use crate::common::db;
use crate::diesel::prelude::*;
use crate::parse::db::entity::characteristic::product_characteristic_int_value::{
    NewProductCharacteristicIntValue, ProductCharacteristicIntValue,
};

pub fn create_if_not_exists(value: i32) -> Option<ProductCharacteristicIntValue> {
    use crate::schema::product_characteristic_int_value;
    let existed_value = get_product_value_by_value(value);
    if existed_value.is_some() {
        return existed_value;
    }

    let connection = &db::establish_connection();
    let new_char_value = NewProductCharacteristicIntValue { value };

    let insert_result: Result<ProductCharacteristicIntValue, diesel::result::Error> =
        diesel::insert_into(product_characteristic_int_value::table)
            .values(&new_char_value)
            .get_result(connection);

    match insert_result {
        Ok(new_char) => Some(new_char),
        Err(e) => {
            if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) = e {
                get_product_value_by_value(value)
            } else {
                sentry::capture_message(
                    format!(
                        "ProductCharacteristicIntValue with value {} can't be created: {:?}",
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

fn get_product_value_by_value(v: i32) -> Option<ProductCharacteristicIntValue> {
    use crate::schema::product_characteristic_int_value::dsl::{
        product_characteristic_int_value, value,
    };
    let connection = &db::establish_connection();

    let filter = value.eq(v);

    let results: Vec<ProductCharacteristicIntValue> = product_characteristic_int_value
        .filter(filter)
        .limit(1)
        .load::<ProductCharacteristicIntValue>(connection)
        .expect("Cannot load product_characteristic_float_value");

    results.into_iter().next()
}

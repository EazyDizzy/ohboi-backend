use lib::diesel::prelude::*;
use lib::diesel::result::{DatabaseErrorKind, Error};
use lib::error_reporting::ReportingContext;
use lib::{db, error_reporting};

use crate::db::entity::characteristic::product_characteristic_string_value::{
    NewProductCharacteristicStringValue, ProductCharacteristicStringValue,
};
use crate::db::Executor;

pub fn create_if_not_exists(value: &str) -> Option<ProductCharacteristicStringValue> {
    use lib::schema::product_characteristic_string_value;
    let existed_value = get_product_value_by_value(value);
    if existed_value.is_some() {
        return existed_value;
    }

    let connection = &db::establish_connection();
    let new_char_value = NewProductCharacteristicStringValue {
        value: value.to_owned(),
    };

    let insert_result: Result<ProductCharacteristicStringValue, diesel::result::Error> =
        diesel::insert_into(product_characteristic_string_value::table)
            .values(&new_char_value)
            .get_result(connection);

    match insert_result {
        Ok(new_char) => Some(new_char),
        Err(e) => {
            if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) = e {
                get_product_value_by_value(value)
            } else {
                error_reporting::warning(
                    format!(
                        "String characteristic with value {} can't be created: {:?}",
                        value, e
                    )
                    .as_str(),
                    &ReportingContext {
                        executor: &Executor::Characteristic,
                        action: "save_characteristic_value::string",
                    },
                );
                None
            }
        }
    }
}

fn get_product_value_by_value(v: &str) -> Option<ProductCharacteristicStringValue> {
    use lib::schema::product_characteristic_string_value::dsl::{
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

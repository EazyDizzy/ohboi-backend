use bigdecimal::BigDecimal;
use lib::diesel::result::{DatabaseErrorKind, Error};

use lib::{db, local_sentry};
use lib::diesel::prelude::*;
use crate::db::entity::characteristic::product_characteristic_float_value::{
    NewProductCharacteristicFloatValue, ProductCharacteristicFloatValue,
};

pub fn create_if_not_exists(value: f32) -> Option<ProductCharacteristicFloatValue> {
    use lib::schema::product_characteristic_float_value;
    let existed_value = get_product_value_by_value(value);
    if existed_value.is_some() {
        return existed_value;
    }

    let connection = &db::establish_connection();
    let new_char_value = NewProductCharacteristicFloatValue {
        value: BigDecimal::from(value),
    };

    let insert_result: Result<ProductCharacteristicFloatValue, diesel::result::Error> =
        diesel::insert_into(product_characteristic_float_value::table)
            .values(&new_char_value)
            .get_result(connection);

    match insert_result {
        Ok(new_char) => Some(new_char),
        Err(e) => {
            if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) = e {
                get_product_value_by_value(value)
            } else {
                local_sentry::capture_message(
                    format!(
                        "ProductCharacteristicFloatValue with value {} can't be created: {:?}",
                        value, e
                    )
                    .as_str(),
                    local_sentry::Level::Warning,
                );
                None
            }
        }
    }
}

fn get_product_value_by_value(v: f32) -> Option<ProductCharacteristicFloatValue> {
    use lib::schema::product_characteristic_float_value::dsl::{
        product_characteristic_float_value, value,
    };
    let connection = &db::establish_connection();

    let filter = value.eq(BigDecimal::from(v));

    let results: Vec<ProductCharacteristicFloatValue> = product_characteristic_float_value
        .filter(filter)
        .limit(1)
        .load::<ProductCharacteristicFloatValue>(connection)
        .expect("Cannot load product_characteristic_float_value");

    results.into_iter().next()
}

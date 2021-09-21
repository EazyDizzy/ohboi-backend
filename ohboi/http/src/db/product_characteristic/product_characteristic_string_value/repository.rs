use lib::db;
use lib::diesel::prelude::*;
use lib::schema::product_characteristic_string_value;

use crate::db::product_characteristic::product_characteristic_string_value::ProductCharacteristicStringValue;

pub fn get_ids_of_values(values: Vec<String>) -> Vec<ProductCharacteristicStringValue> {
    use lib::schema::product_characteristic_string_value::dsl::value;
    let connection = &db::establish_connection();
    let filter = value.eq_any(values);

    product_characteristic_string_value::table
        .filter(filter)
        .load::<ProductCharacteristicStringValue>(connection)
        .expect("Cannot load product product_characteristic_string_value")
}

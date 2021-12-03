use crate::db;
use crate::diesel::prelude::*;
use crate::schema::product_characteristic_enum_value;

use crate::db::product_characteristic::product_characteristic_string_value::ProductCharacteristicStringValue;

pub fn get_ids_of_values(values: Vec<String>) -> Vec<ProductCharacteristicStringValue> {
    use crate::schema::product_characteristic_enum_value::columns::value;
    let connection = &db::establish_connection();
    let filter = value.eq_any(values);

    product_characteristic_enum_value::table
        .filter(filter)
        .load::<ProductCharacteristicStringValue>(connection)
        .expect("Cannot load product product_characteristic_string_value")
}

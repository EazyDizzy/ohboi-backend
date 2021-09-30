use crate::db;
use crate::db::product_characteristic::entity::ProductCharacteristic;
use crate::db::product_characteristic::product_characteristic_enum_value::ProductCharacteristicEnumValue;
use crate::db::product_characteristic::product_characteristic_float_value::ProductCharacteristicFloatValue;
use crate::db::product_characteristic::product_characteristic_string_value::ProductCharacteristicStringValue;
use crate::diesel::prelude::*;
use crate::schema::product_characteristic;
use crate::schema::product_characteristic_enum_value;
use crate::schema::product_characteristic_float_value;
use crate::schema::product_characteristic_string_value;

pub fn get_mapped_float_values(
    values: &[ProductCharacteristic],
) -> Vec<ProductCharacteristicFloatValue> {
    use crate::schema::product_characteristic_float_value::columns::id;
    let connection = &db::establish_connection();
    let ids: Vec<i32> = values.iter().map(|v| v.value_id).collect();
    let filter = id.eq_any(ids);

    product_characteristic_float_value::table
        .filter(filter)
        .load::<ProductCharacteristicFloatValue>(connection)
        .expect("Cannot load product product_characteristic_float_value")
}

pub fn get_mapped_string_values(
    values: &[ProductCharacteristic],
) -> Vec<ProductCharacteristicStringValue> {
    use crate::schema::product_characteristic_string_value::columns::id;
    let connection = &db::establish_connection();
    let ids: Vec<i32> = values.iter().map(|v| v.value_id).collect();
    let filter = id.eq_any(ids);

    product_characteristic_string_value::table
        .filter(filter)
        .load::<ProductCharacteristicStringValue>(connection)
        .expect("Cannot load product product_characteristic_string_value")
}
pub fn get_mapped_enum_values(values: &[ProductCharacteristic]) -> Vec<ProductCharacteristicEnumValue> {
    use crate::schema::product_characteristic_enum_value::columns::id;
    let connection = &db::establish_connection();
    let ids: Vec<i32> = values.iter().map(|v| v.value_id).collect();
    let filter = id.eq_any(ids);

    product_characteristic_enum_value::table
        .filter(filter)
        .load::<ProductCharacteristicEnumValue>(connection)
        .expect("Cannot load product product_characteristic_enum_value")
}

pub fn get_product_characteristics(id: i32) -> Vec<ProductCharacteristic> {
    use crate::schema::product_characteristic::columns::product_id;
    let connection = &db::establish_connection();

    let filter = product_id.eq(id);

    product_characteristic::table
        .filter(filter)
        .load::<ProductCharacteristic>(connection)
        .expect("Cannot load product characteristics")
}

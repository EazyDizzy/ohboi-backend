use bigdecimal::ToPrimitive;

use lib::db;
use lib::dto::characteristic::TypedCharacteristic;
use lib::diesel::prelude::*;
use crate::db::product_characteristic::characteristic_id::get_characteristic_by_id;
use crate::db::product_characteristic::entity::ProductCharacteristic;
use crate::db::product_characteristic::product_characteristic_enum_value::ProductCharacteristicEnumValue;
use crate::db::product_characteristic::product_characteristic_float_value::ProductCharacteristicFloatValue;
use crate::db::product_characteristic::product_characteristic_string_value::ProductCharacteristicStringValue;
use crate::dto::product::*;
use lib::schema::product_characteristic;
use lib::schema::product_characteristic_enum_value;
use lib::schema::product_characteristic_float_value;
use lib::schema::product_characteristic_string_value;

pub fn get_all_characteristics_of_product(product_id: i32) -> ProductCharacteristicsMapped {
    let product_characteristics = get_product_characteristics(product_id);
    let mut float_characteristics = vec![];
    let mut int_characteristics = vec![];
    let mut string_characteristics = vec![];
    let mut enum_characteristics = vec![];

    for product_char in product_characteristics {
        let characteristic = get_characteristic_by_id(product_char.characteristic_id);
        match characteristic.unwrap() {
            TypedCharacteristic::Float(_) => {
                float_characteristics.push(product_char);
            }
            TypedCharacteristic::Int(_) => {
                int_characteristics.push(product_char);
            }
            TypedCharacteristic::String(_) => {
                string_characteristics.push(product_char);
            }
            TypedCharacteristic::Enum(_) => {
                enum_characteristics.push(product_char);
            }
        }
    }

    let all_db_float_values = get_mapped_float_values(&float_characteristics);
    let all_db_string_values = get_mapped_string_values(&string_characteristics);
    let all_db_enum_values = get_mapped_enum_values(&enum_characteristics);

    let float_values = float_characteristics
        .into_iter()
        .map(|v| CharacteristicFloatValue {
            characteristic_id: v.characteristic_id,
            value: all_db_float_values
                .iter()
                .find(|p| p.id == v.value_id)
                .unwrap()
                .value
                .to_f32()
                .unwrap(),
        })
        .collect();
    let int_values = int_characteristics
        .into_iter()
        .map(|v| CharacteristicIntValue {
            characteristic_id: v.characteristic_id,
            value: v.value_id,
        })
        .collect();
    let string_values = string_characteristics
        .into_iter()
        .map(|v| CharacteristicStringValue {
            characteristic_id: v.characteristic_id,
            value: all_db_string_values
                .iter()
                .find(|p| p.id == v.value_id)
                .unwrap()
                .value.clone(),
        })
        .collect();
    let enum_values = enum_characteristics
        .into_iter()
        .map(|v| CharacteristicEnumValue {
            characteristic_id: v.characteristic_id,
            value: all_db_enum_values
                .iter()
                .find(|p| p.id == v.value_id)
                .unwrap()
                .value.clone(),
        })
        .collect();

    ProductCharacteristicsMapped {
        int: int_values,
        float: float_values,
        string: string_values,
        enums: enum_values,
    }
}

fn get_mapped_float_values(
    values: &Vec<ProductCharacteristic>,
) -> Vec<ProductCharacteristicFloatValue> {
    use lib::schema::product_characteristic_float_value::dsl::id;
    let connection = &db::establish_connection();
    let ids: Vec<i32> = values.into_iter().map(|v| v.value_id).collect();
    let filter = id.eq_any(ids);

    product_characteristic_float_value::table
        .filter(filter)
        .load::<ProductCharacteristicFloatValue>(connection)
        .expect("Cannot load product product_characteristic_float_value")
}
fn get_mapped_string_values(
    values: &Vec<ProductCharacteristic>,
) -> Vec<ProductCharacteristicStringValue> {
    use lib::schema::product_characteristic_string_value::dsl::id;
    let connection = &db::establish_connection();
    let ids: Vec<i32> = values.into_iter().map(|v| v.value_id).collect();
    let filter = id.eq_any(ids);

    product_characteristic_string_value::table
        .filter(filter)
        .load::<ProductCharacteristicStringValue>(connection)
        .expect("Cannot load product product_characteristic_string_value")
}
fn get_mapped_enum_values(
    values: &Vec<ProductCharacteristic>,
) -> Vec<ProductCharacteristicEnumValue> {
    use lib::schema::product_characteristic_enum_value::dsl::id;
    let connection = &db::establish_connection();
    let ids: Vec<i32> = values.into_iter().map(|v| v.value_id).collect();
    let filter = id.eq_any(ids);

    product_characteristic_enum_value::table
        .filter(filter)
        .load::<ProductCharacteristicEnumValue>(connection)
        .expect("Cannot load product product_characteristic_enum_value")
}

fn get_product_characteristics(id: i32) -> Vec<ProductCharacteristic> {
    use lib::schema::product_characteristic::dsl::product_id;
    let connection = &db::establish_connection();

    let filter = product_id.eq(id);

    product_characteristic::table
        .filter(filter)
        .load::<ProductCharacteristic>(connection)
        .expect("Cannot load product characteristics")
}

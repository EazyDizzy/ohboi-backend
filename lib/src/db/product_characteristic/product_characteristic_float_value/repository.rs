use bigdecimal::BigDecimal;

use crate::db;
use crate::diesel::prelude::*;
use crate::schema::product_characteristic_float_value;

use crate::db::product_characteristic::product_characteristic_float_value::ProductCharacteristicFloatValue;

#[allow(clippy::ptr_arg)]
pub fn get_ids_of_values(values: &Vec<f32>) -> Vec<ProductCharacteristicFloatValue> {
    use crate::schema::product_characteristic_float_value::columns::value;
    let connection = &db::establish_connection();
    let filter = value.eq_any(
        values
            .iter()
            .map(|v| BigDecimal::from(*v))
            .collect::<Vec<BigDecimal>>(),
    );

    product_characteristic_float_value::table
        .filter(filter)
        .load::<ProductCharacteristicFloatValue>(connection)
        .expect("Cannot load product product_characteristic_float_value")
}

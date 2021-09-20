use bigdecimal::BigDecimal;

use lib::schema::product_characteristic_float_value;

#[derive(Queryable)]
pub struct ProductCharacteristicFloatValue {
    pub id: i32,
    pub value: BigDecimal,
}

#[derive(Insertable)]
#[table_name = "product_characteristic_float_value"]
pub struct NewProductCharacteristicFloatValue {
    pub value: BigDecimal,
}

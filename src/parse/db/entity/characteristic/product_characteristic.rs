use serde::Serialize;

use crate::schema::product_characteristic;

#[derive(Serialize, Queryable)]
pub struct ProductCharacteristic {
    pub id: i32,
    pub product_id: i32,
    pub characteristic_id: i32,
    pub value_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "product_characteristic"]
pub struct NewProductCharacteristic {
    pub product_id: i32,
    pub characteristic_id: i16,
    pub value_id: i32,
}

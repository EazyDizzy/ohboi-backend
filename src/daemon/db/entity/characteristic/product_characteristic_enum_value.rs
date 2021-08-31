use serde::Serialize;
use crate::schema::product_characteristic_enum_value;


#[derive(Serialize, Queryable)]
pub struct ProductCharacteristicEnumValue {
    pub id: i32,
    pub value: String,
}

#[derive(Insertable, Debug)]
#[table_name = "product_characteristic_enum_value"]
pub struct NewProductCharacteristicEnumValue {
    pub value: String,
}

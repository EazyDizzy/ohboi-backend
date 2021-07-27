use serde::Serialize;
use crate::schema::category_characteristic;

#[derive(Serialize, Queryable)]
pub struct CategoryCharacteristic {
    pub id: i32,
    pub category_id: i32,
    pub characteristic_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "category_characteristic"]
pub struct NewCategoryCharacteristic {
    pub category_id: i32,
    pub characteristic_id: i32,
}
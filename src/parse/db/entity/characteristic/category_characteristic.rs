use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct CategoryCharacteristic {
    pub id: i32,
    pub category_id: i32,
    pub characteristic_id: i32,
}
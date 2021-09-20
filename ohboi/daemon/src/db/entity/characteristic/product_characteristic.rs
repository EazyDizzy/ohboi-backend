use lib::schema::product_characteristic;

#[derive(Insertable, Debug)]
#[table_name = "product_characteristic"]
pub struct NewProductCharacteristic {
    pub product_id: i32,
    pub characteristic_id: i16,
    pub value_id: i32,
}

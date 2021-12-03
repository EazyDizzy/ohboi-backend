use lib::db;
use lib::diesel::result::{DatabaseErrorKind, Error};
use lib::diesel::RunQueryDsl;
use lib::schema::product_characteristic;

use crate::db::entity::characteristic::product_characteristic::NewProductCharacteristic;

pub fn create_many_if_not_exists(product_chars: &[NewProductCharacteristic]) {
    let connection = &db::establish_connection();

    let insert_result = diesel::insert_into(product_characteristic::table)
        .values(product_chars)
        .execute(connection);

    match insert_result {
        Ok(_) => {}
        Err(e) => {
            println!("batch insert error {:?}", e);
            if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) = e {
                log::info!("{:?} characteristics already exist", product_chars);
            }
        }
    }
}

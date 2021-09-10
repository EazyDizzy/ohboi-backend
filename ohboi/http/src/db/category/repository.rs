use lib::diesel::prelude::*;
use lib::db;
use crate::db::category::entity::Category;

pub fn get_all() -> Vec<Category> {
    use lib::schema::category::dsl::category;

    let connection = &db::establish_connection();

    category
        .load(connection)
        .expect("Error loading source products")
}
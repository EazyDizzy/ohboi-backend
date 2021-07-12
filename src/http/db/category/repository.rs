use crate::diesel::prelude::*;
use crate::common::db;
use crate::http::db::category::entity::Category;

pub fn get_all() -> Vec<Category> {
    use crate::schema::category::dsl::category;

    let connection = &db::establish_connection();

    category
        .load(connection)
        .expect("Error loading source products")
}
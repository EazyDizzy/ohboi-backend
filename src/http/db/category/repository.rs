use crate::diesel::prelude::*;
use crate::http::db;
use crate::http::db::category::entity::Category;

pub fn get_all() -> Vec<Category> {
    use crate::schema::category::dsl::*;

    let connection = &db::establish_connection();

    category
        .load(connection)
        .expect("Error loading source products")
}
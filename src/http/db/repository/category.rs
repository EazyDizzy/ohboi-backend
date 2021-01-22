use crate::http::db;
use crate::http::db::entity::Category;
use crate::diesel::prelude::*;

pub fn get_all() -> Vec<Category> {
    use crate::schema::category::dsl::*;

    let connection = &db::establish_connection();

    category
        .load(connection)
        .expect("Error loading source products")
}
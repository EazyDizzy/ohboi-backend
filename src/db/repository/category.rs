use inflector::Inflector;
use crate::diesel::prelude::*;

use crate::db::entity::{CategorySlug, Category};
use crate::db;

pub fn get_category(name: &CategorySlug) -> Category {
    use crate::schema::category::dsl::*;
    let connection = &db::establish_connection();

    let filter = slug.eq(name.to_string().to_snake_case());

    let results: Vec<Category> = category.filter(filter)
        .limit(1)
        .load::<Category>(connection)
        .expect("Cannot load category");

    results.into_iter().next().unwrap()
}
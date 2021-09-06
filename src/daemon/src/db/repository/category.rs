use inflector::Inflector;

use lib::db;
use lib::diesel::prelude::*;
use crate::db::entity::category::{Category, CategorySlug};

pub fn get_category(name: CategorySlug) -> Category {
    use lib::schema::category::dsl::{category, slug};
    let connection = &db::establish_connection();

    let filter = slug.eq(name.to_string().to_snake_case());

    let results: Vec<Category> = category.filter(filter)
        .limit(1)
        .load::<Category>(connection)
        .expect("Cannot load category");

    results.into_iter().next().unwrap()
}
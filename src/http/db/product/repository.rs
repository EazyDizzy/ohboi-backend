use diesel::{QueryDsl, RunQueryDsl};

use crate::diesel::prelude::*;
use crate::common::db;
use crate::http::db::lower;
use crate::http::db::product::entity::Product;
use crate::http::product::ProductFilters;
use crate::schema::product;
use crate::schema::product::dsl::{category, enabled, id, title};

pub fn get_all_products_of_category(filters: &ProductFilters) -> Vec<Product> {
    let connection = &db::establish_connection();

    let mut query = product::table
        .into_boxed();

    query = query.filter(enabled.eq(true));

    if filters.title.is_some() {
        let requested_title = filters.title.clone().unwrap().to_lowercase();
        query = query.filter(
            lower(title).like(
                ["%", requested_title.as_str(), "%"].join("")
            )
        );
    }

    if filters.category.is_some() {
        query = query.filter(
            category.eq_any(filters.category.clone().unwrap())
        );
    }

    query
        .limit(20)
        .offset((filters.page * 20).into())
        .order(id.asc())
        .load::<Product>(connection)
        .expect("Error loading products")
}

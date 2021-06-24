use diesel::{QueryDsl, RunQueryDsl};

use crate::common::db;
use crate::diesel::prelude::*;
use crate::http::db::lower;
use crate::http::db::product::entity::Product;
use crate::http::product::ProductFilters;
use crate::schema::product;
use crate::schema::product::dsl::{category, enabled, id, title};
use crate::schema::source_product;

pub fn get_filtered_products(filters: &ProductFilters) -> Vec<Product> {
    let connection = &db::establish_connection();

    let mut query = product::table.left_join(source_product::table)
        .select((
            product::id,
            product::title,
            product::description,
            product::lowest_price,
            product::highest_price,
            product::images,
            product::category,
            product::enabled,
            product::created_at,
            product::updated_at,
        ))
        .into_boxed();

    query = query.filter(enabled.eq(true))
        .limit(20)
        .offset((filters.page * 20).into())
        .order(id.asc());

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

    if filters.source.is_some() {
        query = query
            .filter(source_product::source_id.eq_any(filters.source.clone().unwrap()));
    }

    query
        .load::<Product>(connection)
        .expect("Error loading products")
}

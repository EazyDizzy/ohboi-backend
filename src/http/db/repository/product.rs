use diesel::{QueryDsl, RunQueryDsl};
use diesel::associations::HasTable;
use inflector::Inflector;

use crate::diesel::prelude::*;
use crate::http::db;
use crate::http::db::entity::lower;
use crate::http::db::entity::Product;
use crate::http::product::ProductFilters;
use crate::schema::product;
use crate::schema::product::dsl::{category, enabled, title};

pub fn get_all_products_of_category(requested_filters: &ProductFilters) -> Vec<Product> {
    let connection = &db::establish_connection();

    let mut sql_filters = product::table.into_boxed();
    let basic_filters = category.eq(requested_filters.category)
                                .and(enabled.eq(true));

    if requested_filters.title.is_some() {
        let requested_title = requested_filters.title.clone().unwrap().to_lowercase();
        sql_filters = sql_filters.filter(
            basic_filters.and(
                lower(title).like(
                    ["%", requested_title.as_str(), "%"].join("")
                )
            )
        );
    }

    sql_filters.limit(20)
               .offset((requested_filters.page * 20).into())
               .load::<Product>(connection)
               .expect("Error loading products")
}

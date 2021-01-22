use diesel::{QueryDsl, RunQueryDsl};

use crate::http::db;
use crate::http::db::entity::Product;
use crate::diesel::prelude::*;

pub fn get_all_products_of_category(product_category: &i32, page: &i32) -> Vec<Product> {
    use crate::schema::product::dsl::*;

    let connection = &db::establish_connection();

    let targets = product.filter(
        category.eq(product_category)
                .and(enabled.eq(true))
    );

    targets.limit(20)
           .offset((page * 20).into())
           .load::<Product>(connection)
           .expect("Error loading products")
}

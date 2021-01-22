use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::http::db;
use crate::http::db::entity::SourceProduct;

pub fn get_all_for_product(requested_product_id: &i32) -> Vec<SourceProduct> {
    use crate::schema::source_product::dsl::*;

    let connection = &db::establish_connection();
    let targets = source_product.filter(
        product_id.eq(requested_product_id)
                  .and(enabled.eq(true))
    );

    targets
        .load::<SourceProduct>(connection)
        .expect("Error loading source products")
}
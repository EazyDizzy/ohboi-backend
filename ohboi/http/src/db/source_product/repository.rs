use lib::diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};

use lib::db;
use crate::db::source_product::entity::SourceProduct;

pub fn get_all_for_product(requested_product_id: i32) -> Vec<SourceProduct> {
    use lib::schema::source_product::dsl::{enabled, product_id, source_product};

    let connection = &db::establish_connection();
    let targets = source_product.filter(
        product_id.eq(requested_product_id)
            .and(enabled.eq(true))
    );

    targets
        .load::<SourceProduct>(connection)
        .expect("Error loading source products")
}
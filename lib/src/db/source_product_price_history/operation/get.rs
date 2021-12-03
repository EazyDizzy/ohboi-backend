use diesel::{ExpressionMethods, QueryDsl};

use crate::db;
use crate::db::source_product_price_history::entity::SourceProductPriceHistory;
use crate::diesel::RunQueryDsl;
use crate::schema::source_product_price_history::dsl::source_product_price_history;

pub fn get_all_for(sought_product_id: i32) -> Vec<SourceProductPriceHistory> {
    use crate::schema::source_product_price_history::dsl::product_id;

    let connection = &db::establish_connection();

    let target = source_product_price_history.filter(product_id.eq(sought_product_id));
    target
        .load::<SourceProductPriceHistory>(connection)
        .expect("Error loading exchange rate")
}

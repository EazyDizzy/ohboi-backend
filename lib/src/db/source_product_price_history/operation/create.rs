use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::Utc;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::db;
use crate::db::source_product_price_history::entity::{
    NewSourceProductPriceHistory, SourceProductPriceHistory,
};
use crate::schema::source_product_price_history;

pub fn add_price_to_history_if_not_exists(
    price_product_id: i32,
    price_source_id: i32,
    price_external_id: &str,
    price_value: BigDecimal,
) {
    use crate::schema::source_product_price_history::dsl::{
        external_id, id, product_id, source_id, source_product_price_history,
    };

    let connection = &db::establish_connection();

    let target = source_product_price_history.filter(
        product_id
            .eq(&price_product_id)
            .and(source_id.eq(&price_source_id))
            .and(external_id.eq(price_external_id)),
    );

    let results: Vec<SourceProductPriceHistory> = target
        .order(id.desc())
        .limit(1)
        .load::<SourceProductPriceHistory>(connection)
        .expect("Error loading product");

    if results.is_empty() {
        create(
            price_product_id,
            price_source_id,
            price_external_id,
            price_value,
        );
    } else {
        let last_history_price = results.into_iter().next().unwrap();

        if !last_history_price.price.eq(&price_value) {
            create(
                price_product_id,
                price_source_id,
                price_external_id,
                price_value,
            );
        }
    }
}

fn create(product_id: i32, source_id: i32, external_id: &str, price: BigDecimal) {
    let connection = &db::establish_connection();
    let now = Utc::now();

    let new_price_history = NewSourceProductPriceHistory {
        source_id,
        product_id,
        external_id,
        price: BigDecimal::from(price.to_f64().unwrap()),
        created_at: &now.naive_utc(),
    };

    diesel::insert_into(source_product_price_history::table)
        .values(&new_price_history)
        .execute(connection)
        .expect("Error saving new history price");
}

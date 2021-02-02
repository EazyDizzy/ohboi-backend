use crate::schema::source_product_price_history;
use crate::parse::db;
use diesel::{ExpressionMethods, QueryDsl, BoolExpressionMethods, RunQueryDsl};
use crate::parse::db::entity::{NewSourceProductPriceHistory, NewSourceProduct, SourceProductPriceHistory};
use chrono::Utc;
use bigdecimal::{BigDecimal, ToPrimitive};

pub fn add_to_history_if_not_exists(source_product: &NewSourceProduct) {
    use crate::schema::source_product_price_history::dsl::*;

    let connection = &db::establish_connection();

    let target = source_product_price_history.filter(
        product_id.eq(&source_product.product_id)
            .and(source_id.eq(&source_product.source_id))
            .and(external_id.eq(&source_product.external_id))
    );

    let results: Vec<SourceProductPriceHistory> = target
        .order(id.desc())
        .limit(1)
        .load::<SourceProductPriceHistory>(connection)
        .expect("Error loading product");

    if results.len() == 0 {
        create(&source_product);
    } else {
        let last_history_price = results.into_iter().next().unwrap();

        if !last_history_price.price.eq(&source_product.price) {
            create(&source_product);
        }
    }
}

fn create(source_product: &NewSourceProduct) {
    let connection = &db::establish_connection();
    let now = Utc::now();

    let new_price_history = NewSourceProductPriceHistory {
        source_id: source_product.source_id,
        product_id: source_product.product_id,
        external_id: source_product.external_id,
        price: BigDecimal::from(source_product.price.to_f64().unwrap()),
        created_at: &now.naive_utc(),
    };

    diesel::insert_into(source_product_price_history::table)
        .values(&new_price_history)
        .execute(connection)
        .expect("Error saving new history price");
}
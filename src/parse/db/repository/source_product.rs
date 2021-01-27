use std::borrow::Borrow;

use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::Utc;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::parse::db;
use crate::parse::db::entity::{NewSourceProduct, Product, SourceName, SourceProduct};
use crate::parse::db::repository::product::update_lowest_price;
use crate::parse::db::repository::source::get_source;
use crate::parse::db::repository::source_product_price_history::add_to_history_if_not_exists;
use crate::parse::parsed_product::ParsedProduct;
use crate::schema::source_product;

pub fn get_by_source_and_external_id(source: &SourceName, expected_external_id: String) -> Option<SourceProduct> {
    use crate::schema::source_product::dsl::*;

    let connection = &db::establish_connection();
    let source = get_source(source);

    let target = source_product.filter(
        source_id.eq(source.id)
                 .and(external_id.eq(expected_external_id))
    );

    let results: Vec<SourceProduct> = target
        .limit(1)
        .load::<SourceProduct>(connection)
        .expect("Error loading product");

    results.into_iter().next()
}

pub fn link_to_product(product: &Product, parsed_product: &ParsedProduct, source: &SourceName) {
    let source = get_source(source);

    let now = Utc::now();
    let new_link = NewSourceProduct {
        source_id: source.id,
        product_id: product.id,
        enabled: parsed_product.available,
        price: BigDecimal::from(parsed_product.price),
        updated_at: &now.naive_utc(),
        external_id: &parsed_product.external_id,
    };

    create_if_not_exists(&new_link);

    let fresh_product_is_cheaper = parsed_product.price.lt(
        product.lowest_price.to_f64().unwrap().borrow()
    );
    let current_product_has_zero_price = product.lowest_price.to_f64().unwrap().eq(0.to_f64().unwrap().borrow());
    if fresh_product_is_cheaper || current_product_has_zero_price {
        update_lowest_price(&product.id, parsed_product.price);
    }

    add_to_history_if_not_exists(&new_link);
}

fn create_if_not_exists(new_product: &NewSourceProduct) {
    let connection = &db::establish_connection();

    diesel::insert_into(source_product::table)
        .values(new_product)
        .on_conflict((source_product::source_id, source_product::product_id, source_product::external_id))
        .do_update()
        .set((
            source_product::price.eq(&new_product.price),
            source_product::updated_at.eq(&new_product.updated_at),
            source_product::enabled.eq(&new_product.enabled)
        ))
        .execute(connection)
        .expect("Error saving new source product");
}
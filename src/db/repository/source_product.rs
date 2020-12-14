use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::Utc;
use diesel::{RunQueryDsl, ExpressionMethods};
use crate::db;

use crate::parse::parsed_product::ParsedProduct;
use crate::db::entity::{SourceName, CategorySlug, NewSourceProduct};
use crate::db::repository::product::{create_if_not_exists as create_product, update_lowest_price};
use crate::db::repository::source::get_source;
use crate::schema::source_product;
use std::borrow::Borrow;
use crate::db::repository::source_product_price_history::add_to_history_if_not_exists;

pub fn link_to_product(parsed_product: &ParsedProduct, source: &SourceName, product_category: &CategorySlug) {
    let product = create_product(parsed_product, product_category);
    let source = get_source(source);

    let now = Utc::now();
    let new_link = NewSourceProduct {
        source_id: source.id,
        product_id: product.id,
        enabled: parsed_product.available,
        price: BigDecimal::from(parsed_product.price),
        updated_at: &now.naive_utc(),
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
        .on_conflict((source_product::source_id, source_product::product_id))
        .do_update()
        .set((
            source_product::price.eq(&new_product.price),
            source_product::updated_at.eq(&new_product.updated_at),
            source_product::enabled.eq(&new_product.enabled)
        ))
        .execute(connection)
        .expect("Error saving new source product");
}
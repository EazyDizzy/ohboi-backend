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

pub fn link_to_product(parsed_product: &ParsedProduct, source: &SourceName, product_category: &CategorySlug) {
    let product = create_product(parsed_product, product_category);
    let source = get_source(source);

    let now = Utc::now();
    let new_link = NewSourceProduct {
        source_id: source.id,
        product_id: product.id,
        price: BigDecimal::from(parsed_product.price),
        updated_at: &now.naive_utc(),
    };

    create_if_not_exists(new_link);

    if parsed_product.price.lt(product.lowest_price.to_f64().unwrap().borrow()) {
        update_lowest_price(&product.id, parsed_product.price);
    }
}

fn create_if_not_exists(new_product: NewSourceProduct) {
    let connection = &db::establish_connection();

    diesel::insert_into(source_product::table)
        .values(&new_product)
        .on_conflict((source_product::source_id, source_product::product_id))
        .do_update()
        .set((
            source_product::price.eq(&new_product.price)
            , source_product::updated_at.eq(&new_product.updated_at)
        ))
        .execute(connection)
        .expect("Error saving new source product");
}
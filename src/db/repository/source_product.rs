use bigdecimal::BigDecimal;
use chrono::Utc;
use diesel::RunQueryDsl;
use crate::db;

use crate::parse::parsed_product::ParsedProduct;
use crate::db::entity::{SourceName, CategorySlug, NewSourceProduct};
use crate::db::repository::product::create_if_not_exists;
use crate::db::repository::source::get_source;
use crate::schema::source_product;

pub fn link_to_product(parsed_product: &ParsedProduct, source: &SourceName, product_category: &CategorySlug) {
    let connection = &db::establish_connection();

    let id_to_connect = create_if_not_exists(parsed_product, product_category);
    let source = get_source(source);

    let now = Utc::now();
    let new_link = NewSourceProduct {
        source_id: source.id,
        product_id: id_to_connect,
        price: BigDecimal::from(parsed_product.price),
        updated_at: &now.naive_utc(),
    };

    diesel::insert_into(source_product::table)
        .values(&new_link)
        .execute(connection)
        .expect("Error saving new source product");
}
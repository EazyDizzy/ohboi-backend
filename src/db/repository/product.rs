use bigdecimal::BigDecimal;
use chrono::Utc;
use diesel::{RunQueryDsl, QueryDsl};

use crate::db;
use crate::db::entity::{NewProduct, Product, CategorySlug};
use crate::schema::product;
use crate::parse::parsed_product::ParsedProduct;
use crate::diesel::prelude::*;
use crate::db::repository::category::get_category;

pub fn create(title: &str, price: f64, images: &Vec<String>, product_category: &CategorySlug) -> Product {
    let connection = &db::establish_connection();
    let now = Utc::now();
    let category = get_category(product_category);

    let new_product = NewProduct {
        category: category.id,
        title,
        images,
        lowest_price: BigDecimal::from(price),
        created_at: &now.naive_utc(),
        updated_at: &now.naive_utc(),
    };

    diesel::insert_into(product::table)
        .values(&new_product)
        .get_result(connection)
        .expect("Error saving new product")
}

pub fn create_if_not_exists(parsed_product: &ParsedProduct, product_category: &CategorySlug) -> i32 {
    use crate::schema::product::dsl::*;

    let connection = &db::establish_connection();

    let filter = title.eq(&parsed_product.title);
    let results: Vec<Product> = product.filter(filter)
        .limit(1)
        .load::<Product>(connection)
        .expect("Error loading products");

    println!("{}", results.len());

    if results.len() == 0 {
        create(
            &parsed_product.title,
            parsed_product.price,
            &vec![parsed_product.image_url.to_string()],
            product_category,
        ).id
    } else {
        let product_to_return = &*results.first().unwrap();

        product_to_return.id
    }
}
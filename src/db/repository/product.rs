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

pub fn create_if_not_exists(parsed_product: &ParsedProduct, product_category: &CategorySlug) -> Product {
    use crate::schema::product::dsl::*;

    let connection = &db::establish_connection();

    let target = title.eq(&parsed_product.title);
    let results: Vec<Product> = product.filter(target)
        .limit(1)
        .load::<Product>(connection)
        .expect("Error loading products");

    if results.len() == 0 {
        create(
            &parsed_product.title,
            parsed_product.price,
            &vec![parsed_product.image_url.to_string()],
            product_category,
        )
    } else {
        results.into_iter().next().unwrap()
    }
}

pub fn update_lowest_price(product_id: &i32, new_price: f64) {
    let now = Utc::now();
    use crate::schema::product::dsl::*;

    let connection = &db::establish_connection();
    let target = product.filter(id.eq(product_id));

    diesel::update(target)
        .set((lowest_price.eq(BigDecimal::from(new_price)), updated_at.eq(&now.naive_utc())))
        .execute(connection)
        .expect("Failed to update product price");
}
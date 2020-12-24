use bigdecimal::{BigDecimal};
use chrono::Utc;
use diesel::{RunQueryDsl, QueryDsl};

use crate::db;
use crate::db::entity::{NewProduct, Product, CategorySlug};
use crate::schema::product;
use crate::parse::parsed_product::{ParsedProduct, AdditionalParsedProductInfo};
use crate::diesel::prelude::*;
use crate::db::repository::category::get_category;

pub fn get_all_products_of_category(product_category: &i32, page: &i32) -> Vec<Product> {
    use crate::schema::product::dsl::*;

    let connection = &db::establish_connection();

    let targets = product.filter(
        category.eq(product_category)
            .and(enabled.eq(true))
    );

    targets.limit(20)
        .offset((page * 20).into())
        .load::<Product>(connection)
        .expect("Error loading products")
}

pub fn update_details(existent_product: &Product, additional_info: &AdditionalParsedProductInfo) {
    use crate::schema::product::dsl::*;

    let connection = &db::establish_connection();
    let target = product.filter(id.eq(existent_product.id));

    diesel::update(target)
        .set((
            description.eq(&additional_info.description),
            images.eq(&additional_info.image_urls),
            enabled.eq(existent_product.enabled || additional_info.available)
        ))
        .execute(connection)
        .expect("Failed to update product price");
}

pub fn create_if_not_exists(parsed_product: &ParsedProduct, product_category: &CategorySlug) -> Product {
    use crate::schema::product::dsl::*;

    let connection = &db::establish_connection();

    let target = product.filter(title.eq(&parsed_product.title));
    let results: Vec<Product> = target
        .limit(1)
        .load::<Product>(connection)
        .expect("Error loading product");

    if results.len() == 0 {
        create(
            &parsed_product,
            product_category,
        )
    } else {
        let current_product = results.into_iter().next().unwrap();
        if parsed_product.available && !current_product.enabled {
            enable_product(&current_product.id);
        }

        current_product
    }
}

pub fn update_lowest_price(product_id: &i32, new_price: f64) {
    use crate::schema::product::dsl::*;
    let now = Utc::now();

    let connection = &db::establish_connection();
    let target = product.filter(id.eq(product_id));

    diesel::update(target)
        .set((lowest_price.eq(BigDecimal::from(new_price)), updated_at.eq(&now.naive_utc())))
        .execute(connection)
        .expect("Failed to update product price");
}

fn create(parsed_product: &ParsedProduct, product_category: &CategorySlug) -> Product {
    let connection = &db::establish_connection();
    let now = Utc::now();
    let category = get_category(product_category);

    let new_product = NewProduct {
        category: category.id,
        title: &parsed_product.title,
        enabled: false,
        lowest_price: BigDecimal::from(parsed_product.price),
        created_at: &now.naive_utc(),
        updated_at: &now.naive_utc(),
    };

    diesel::insert_into(product::table)
        .values(&new_product)
        .get_result(connection)
        .expect("Error saving new product")
}

fn enable_product(product_id: &i32) {
    use crate::schema::product::dsl::*;
    let connection = &db::establish_connection();
    let now = Utc::now();

    let target = product.filter(id.eq(product_id));

    diesel::update(target)
        .set((enabled.eq(true), updated_at.eq(&now.naive_utc())))
        .execute(connection)
        .expect("Failed to enable product");
}
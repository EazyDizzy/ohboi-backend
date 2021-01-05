use bigdecimal::BigDecimal;
use chrono::Utc;
use diesel::{QueryDsl, RunQueryDsl};

use crate::db;
use crate::db::entity::{CategorySlug, NewProduct, Product};
use crate::db::repository::category::get_category;
use crate::diesel::prelude::*;
use crate::parse::parsed_product::{AdditionalParsedProductInfo, ParsedProduct};
use crate::schema::product;

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
    let existed_product = get_product_by_title(parsed_product.title.as_str());

    if existed_product.is_none() {
        create(
            &parsed_product,
            product_category,
        )
    } else {
        let current_product = existed_product.unwrap();

        // TODO what if all sub products were disabled
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

    let insert_result = diesel::insert_into(product::table)
        .values(&new_product)
        .get_result(connection);

    if insert_result.is_err() {
        get_product_by_title(parsed_product.title.as_str()).unwrap()
    } else {
        insert_result.unwrap()
    }
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

fn get_product_by_title(product_title: &str) -> Option<Product> {
    use crate::schema::product::dsl::*;

    let connection = &db::establish_connection();

    let target = product.filter(title.eq(product_title));
    let results: Vec<Product> = target
        .limit(1)
        .load::<Product>(connection)
        .expect("Error loading product");

    if results.len() == 0 {
        None
    } else {
        results.into_iter().next()
    }
}
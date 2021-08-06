use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::Utc;
use diesel::{sql_query, QueryDsl, RunQueryDsl};

use crate::common::db;
use crate::diesel::prelude::*;
use crate::parse::db::entity::category::CategorySlug;
use crate::parse::db::entity::product::{NewProduct, Product};
use crate::parse::db::repository::category::get_category;
use crate::parse::db::repository::characteristic::product_characteristic_float_value;
use crate::parse::dto::characteristic::float_characteristic::FloatCharacteristic;
use crate::parse::dto::parsed_product::{
    AdditionalParsedProductInfo, InternationalParsedProduct, TypedCharacteristic,
};
use crate::schema::product;

pub fn add_image_to_product_details(existent_product_id: i32, file_path: &str) {
    let connection = &db::establish_connection();

    sql_query(format!(
        "UPDATE product SET images = array_append(images, '{file_path}') WHERE id = {id}",
        file_path = file_path,
        id = existent_product_id
    ))
    .execute(connection)
    .expect("Failed pushing new image to the list");
    // TODO enable?
}

pub fn update_details(existent_product: &Product, additional_info: &AdditionalParsedProductInfo) {
    use crate::schema::product::dsl::{description, enabled, id, images, product};

    let connection = &db::establish_connection();
    let target = product.filter(id.eq(existent_product.id));
    additional_info.characteristics.iter().for_each(|tc| {
        let value_id: Option<i32> = match tc {
            TypedCharacteristic::Float(v) => {
                let char_value = v.value();
                let product_value =
                    product_characteristic_float_value::create_if_not_exists(char_value);
                product_value.and_then(|v| Some(v.id))
            }
            TypedCharacteristic::Int(_) => None,
            TypedCharacteristic::String(_) => None,
            TypedCharacteristic::Enum(_) => None,
        };
    });

    panic!("her");
    diesel::update(target)
        .set((
            description.eq(&additional_info.description),
            images.eq(&additional_info.image_urls),
            enabled.eq((existent_product.enabled || additional_info.available)
                && !additional_info.image_urls.is_empty()
                && !additional_info.description.is_empty()),
        ))
        .execute(connection)
        .expect("Failed to update product price");
}

pub fn create_if_not_exists(
    parsed_product: &InternationalParsedProduct,
    product_category: CategorySlug,
) -> Product {
    let existed_product = get_product_by_title(parsed_product.title.as_str());

    existed_product.map_or_else(
        || create(parsed_product, product_category),
        |current_product| {
            if parsed_product.available && !current_product.enabled {
                enable_product(current_product.id);
            }

            current_product
        },
    )
}

pub fn update_price_range_if_needed(product_id: i32, new_price: f64) {
    use crate::schema::product::dsl::{highest_price, id, lowest_price, product, updated_at};
    let now = Utc::now();

    let connection = &db::establish_connection();
    let existing_product = get_product_by_id(product_id).unwrap();
    let current_lowest_price = existing_product.lowest_price.to_f64().unwrap();
    let current_highest_price = existing_product.highest_price.to_f64().unwrap();

    let fresh_product_is_cheaper = new_price.lt(&current_lowest_price);
    let current_product_has_zero_price = current_lowest_price.eq(&0.to_f64().unwrap());
    let should_update_lowest_price = fresh_product_is_cheaper || current_product_has_zero_price;
    let should_update_highest_price = new_price.gt(&current_highest_price);

    let mut new_lowest_price = current_lowest_price;
    let mut new_highest_price = current_highest_price;
    if should_update_lowest_price {
        new_lowest_price = new_price;
    }

    if should_update_highest_price {
        new_highest_price = new_price;
    }

    if should_update_lowest_price || should_update_highest_price {
        let target = product.filter(id.eq(product_id));

        diesel::update(target)
            .set((
                lowest_price.eq(BigDecimal::from(new_lowest_price)),
                highest_price.eq(BigDecimal::from(new_highest_price)),
                updated_at.eq(&now.naive_utc()),
            ))
            .execute(connection)
            .expect("Failed to update product price");
    }
}

fn create(parsed_product: &InternationalParsedProduct, product_category: CategorySlug) -> Product {
    let connection = &db::establish_connection();
    let now = Utc::now();
    let category = get_category(product_category);

    let new_product = NewProduct {
        category: category.id,
        title: &parsed_product.title,
        enabled: false,
        lowest_price: BigDecimal::from(parsed_product.price),
        highest_price: BigDecimal::from(parsed_product.price),
        created_at: &now.naive_utc(),
        updated_at: &now.naive_utc(),
    };

    let insert_result = diesel::insert_into(product::table)
        .values(&new_product)
        .get_result(connection);

    if let Ok(product) = insert_result {
        product
    } else {
        get_product_by_title(parsed_product.title.as_str()).unwrap()
    }
}

fn enable_product(product_id: i32) {
    use crate::schema::product::dsl::{enabled, id, product, updated_at};
    let connection = &db::establish_connection();
    let now = Utc::now();

    let target = product.filter(id.eq(product_id));

    diesel::update(target)
        .set((enabled.eq(true), updated_at.eq(&now.naive_utc())))
        .execute(connection)
        .expect("Failed to enable product");
}

fn get_product_by_title(product_title: &str) -> Option<Product> {
    use crate::schema::product::dsl::{product, title};

    let connection = &db::establish_connection();

    let target = product.filter(title.eq(product_title));
    let results: Vec<Product> = target
        .limit(1)
        .load::<Product>(connection)
        .expect("Error loading product");

    results.into_iter().next()
}

fn get_product_by_id(product_id: i32) -> Option<Product> {
    use crate::schema::product::dsl::{id, product};

    let connection = &db::establish_connection();

    let target = product.filter(id.eq(product_id));
    let results: Vec<Product> = target
        .limit(1)
        .load::<Product>(connection)
        .expect("Error loading product");

    results.into_iter().next()
}

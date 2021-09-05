use bigdecimal::BigDecimal;

use lib::db;
use lib::db::lower;
use lib::db::repository::exchange_rate::try_get_exchange_rate_by_code;
use lib::diesel::prelude::*;
use lib::diesel::{QueryDsl, RunQueryDsl};
use lib::schema::product;
use lib::schema::product::dsl::{category, enabled, highest_price, id, lowest_price, title};
use lib::schema::source_product;
use lib::service::currency_converter::convert_from;

use crate::http::db::product::entity::Product;
use crate::http::db::product_characteristic::repository::get_all_characteristics_of_product;
use crate::http::dto::product::ProductInfo;
use crate::http::product::{ProductFilters, ProductParams};
use crate::http::util::product::convert_product_prices;

pub fn get_product_info(params: &ProductParams) -> Option<ProductInfo> {
    let connection = &db::establish_connection();
    use lib::schema::product::dsl::id;

    let targets = product::table.filter(id.eq(params.id).and(enabled.eq(true)));

    let product: Option<Product> = targets
        .load::<Product>(connection)
        .expect("Error loading source products")
        .into_iter()
        .next();

    product.and_then(|mut p| {
        let rate = try_get_exchange_rate_by_code(params.currency);
        convert_product_prices(&mut p, rate);
        let characteristics = get_all_characteristics_of_product(p.id);
        Some(ProductInfo {
            id: p.id,
            title: p.title,
            description: p.description,
            lowest_price: p.lowest_price,
            highest_price: p.highest_price,
            images: p.images,
            category: p.category,
            characteristics,
        })
    })
}

pub fn get_filtered_products(filters: &ProductFilters) -> Vec<Product> {
    let connection = &db::establish_connection();

    // TODO join conditionally
    let mut query = product::table
        .left_join(source_product::table)
        .select((
            product::id,
            product::title,
            product::description,
            product::lowest_price,
            product::highest_price,
            product::images,
            product::category,
            product::enabled,
            product::created_at,
            product::updated_at,
        ))
        .into_boxed();

    query = query
        .filter(enabled.eq(true))
        .limit(20)
        .offset((filters.page * 20).into())
        .order(id.asc());

    if let Some(filtered_title) = &filters.title {
        let requested_title = filtered_title.to_lowercase();
        query = query.filter(lower(title).like(["%", requested_title.as_str(), "%"].join("")));
    }

    if let Some(filtered_category) = &filters.category {
        query = query.filter(category.eq_any(filtered_category));
    }

    if let Some(source) = &filters.source {
        query = query.filter(source_product::source_id.eq_any(source));
    }

    if let Some(min_price) = filters.min_price {
        query = query
            .filter(highest_price.ge(BigDecimal::from(convert_from(min_price, filters.currency))));
    }
    if let Some(max_price) = filters.max_price {
        query = query
            .filter(lowest_price.le(BigDecimal::from(convert_from(max_price, filters.currency))));
    }

    query
        .load::<Product>(connection)
        .expect("Error loading products")
}

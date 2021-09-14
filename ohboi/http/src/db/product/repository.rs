use bigdecimal::BigDecimal;

use lib::db;
use lib::db::lower;
use lib::db::repository::exchange_rate::try_get_exchange_rate_by_code;
use lib::diesel::{QueryDsl, RunQueryDsl, sql_query};
use lib::diesel::prelude::*;
use lib::schema::product;
use lib::schema::product::dsl::{category, enabled, highest_price, id, lowest_price, title};
use lib::schema::source_product;
use lib::service::currency_converter::convert_from;

use crate::db::product::entity::Product;
use crate::db::product_characteristic::repository::get_all_characteristics_of_product;
use crate::dto::product::ProductInfo;
use crate::endpoint::product::{ProductFilters, ProductParams};
use crate::util::product::convert_product_prices;

pub fn get_product_info(params: &ProductParams) -> Option<ProductInfo> {
    let connection = &db::establish_connection();

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
    let mut query = r"SELECT
    DISTINCT(p.id), p.title, p.description, p.lowest_price, p.highest_price, p.images, p.category, p.enabled, p.created_at, p.updated_at
    FROM product p".to_owned();

    let mut filter = "WHERE p.enabled = true".to_owned();
    let mut joins = "".to_owned();

    // TODO params sanitization
    if let Some(filtered_title) = &filters.title {
        let requested_title = filtered_title.to_lowercase();
        filter.push_str(&format!(" AND LOWER(p.title) LIKE '%{}%' ", requested_title));
    }

    if let Some(filtered_category) = &filters.category {
        let ids: Vec<String> = filtered_category.into_iter().map(|v| v.to_string()).collect();
        filter.push_str(&format!(
            " AND p.category IN ({}) ",
            ids.join(",")
        ));
    }

    if let Some(source) = &filters.source {
        let ids: Vec<String> = source.into_iter().map(|v| v.to_string()).collect();
        joins.push_str(" LEFT JOIN source_product sp on p.id = sp.product_id ");
        filter.push_str(&format!(
            " AND sp.source_id IN ({}) ",
            ids.join(",")
        ));
    }

    if let Some(min_price) = filters.min_price {
        filter.push_str(&format!(" AND p.highest_price >= {} ", convert_from(min_price, filters.currency)));
    }

    if let Some(max_price) = filters.max_price {
        filter.push_str(&format!(" AND p.lowest_price <= {} ", convert_from(max_price, filters.currency)));
    }

    query.push_str(&joins);
    query.push_str(&filter);
    query.push_str(" ORDER BY p.id ASC LIMIT 20");

    sql_query(query)
        .load::<Product>(connection)
        .expect("Error loading products")
}

pub fn get_filtered_products2(filters: &ProductFilters) -> Vec<Product> {
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
        .distinct_on(product::id)
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

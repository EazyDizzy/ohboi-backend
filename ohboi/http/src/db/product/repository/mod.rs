mod search;
pub use search::*;
use std::fmt::{Debug, Display};

use bigdecimal::{BigDecimal, ToPrimitive};

use lib::db;
use lib::db::lower;
use lib::db::repository::exchange_rate::try_get_exchange_rate_by_code;
use lib::diesel::{QueryDsl, RunQueryDsl, sql_query};
use lib::diesel::expression::sql_literal::sql;
use lib::diesel::prelude::*;
use lib::schema::product;
use lib::schema::product::dsl::{category, enabled, highest_price, id, lowest_price, title};
use lib::schema::product_characteristic;
use lib::schema::source_product;
use lib::service::currency_converter::convert_from;

use crate::db::product::entity::Product;
use crate::db::product_characteristic::product_characteristic_float_value;
use crate::db::product_characteristic::repository::get_all_characteristics_of_product;
use crate::dto::product::{CharacteristicIntValue, ProductInfo, CharacteristicFloatValue};
use crate::endpoint::product::{ProductFilters, ProductParams};
use crate::util::product::convert_product_prices;
use lib::diesel::query_source::joins::{JoinOn, Join};

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

pub fn get_filtered_products2(filters: &ProductFilters) -> Vec<Product> {
    let connection = &db::establish_connection();

    // TODO join conditionally
    let mut query = product::table
        .inner_join(source_product::table)
        .inner_join(product_characteristic::table)
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
        .group_by(product::id)
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

    if let Some(characteristics) = filters.characteristics.clone() {
        let int_char_filter = get_int_char_filter(characteristics.int);
        if let Some(int_char_filter) = int_char_filter {
            query = query.filter(sql(&int_char_filter));
        }

        let float_char_filter = get_float_char_filter(characteristics.float);
        if let Some(float_char_filter) = float_char_filter {
            query = query.filter(sql(&float_char_filter));
        }

    }

    query
        .load::<Product>(connection)
        .expect("Error loading products")
}

fn get_float_char_filter(chars: Vec<CharacteristicFloatValue>) -> Option<String> {
    if chars.is_empty() {
        return None;
    }

    let float_value_ids = product_characteristic_float_value::get_ids_of_values(
        chars.iter().map(|v| v.value).collect(),
    );

    let char_tuples = chars
        .iter()
        .map(|char| {
            (
                char.characteristic_id,
                float_value_ids
                    .iter()
                    .find(|v| v.value.to_f32().unwrap() == char.value)
                    .expect("Not-existed in db float value")
                    .id,
            )
        })
        .collect();

    Some(get_char_filter_sql(char_tuples))
}

fn get_int_char_filter(chars: Vec<CharacteristicIntValue>) -> Option<String> {
    if chars.is_empty() {
        return None;
    }

    let char_tuples = chars
        .iter()
        .map(|v| (v.characteristic_id, v.value))
        .collect();

    Some(get_char_filter_sql(char_tuples))
}

fn get_char_filter_sql<V>(chars: Vec<(i16, V)>) -> String
    where
        V: Display,
{
    let raw_filters: Vec<String> = chars
        .iter()
        .map(|(characteristic_id, value)| {
            format!(
                "(product_characteristic.characteristic_id = {} AND product_characteristic.value_id = {})",
                characteristic_id,
                value
            )
        })
        .collect();

    vec!["(", &raw_filters.join(" OR "), ")"].concat()
}
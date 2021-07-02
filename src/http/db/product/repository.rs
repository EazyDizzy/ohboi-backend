use bigdecimal::BigDecimal;
use diesel::{QueryDsl, RunQueryDsl};

use crate::common::db;
use crate::common::service::currency_converter::convert_from;
use crate::diesel::prelude::*;
use crate::http::db::lower;
use crate::http::db::product::entity::Product;
use crate::http::product::ProductFilters;
use crate::schema::product;
use crate::schema::product::dsl::*;
use crate::schema::source_product;

pub fn get_filtered_products(filters: &ProductFilters) -> Vec<Product> {
    let connection = &db::establish_connection();

    // TODO join conditionally
    let mut query = product::table.left_join(source_product::table)
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

    query = query.filter(enabled.eq(true))
        .limit(20)
        .offset((filters.page * 20).into())
        .order(id.asc());

    if let Some(filtered_title) = &filters.title {
        let requested_title = filtered_title.to_lowercase();
        query = query.filter(
            lower(title).like(
                ["%", requested_title.as_str(), "%"].join("")
            )
        );
    }

    if let Some(filtered_category) = &filters.category {
        query = query.filter(
            category.eq_any(filtered_category)
        );
    }

    if let Some(source) = &filters.source {
        query = query
            .filter(source_product::source_id.eq_any(source));
    }

    if let Some(min_price) = filters.min_price {
        query = query.filter(
            highest_price.ge(
                BigDecimal::from(convert_from(min_price, &filters.currency))
            )
        );
    }
    if let Some(max_price) = filters.max_price {
        query = query.filter(
            lowest_price.le(
                BigDecimal::from(convert_from(max_price, &filters.currency))
            )
        );
    }

    query
        .load::<Product>(connection)
        .expect("Error loading products")
}

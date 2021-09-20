use std::collections::BTreeMap;
use std::fmt::{Debug, Display};

use bigdecimal::ToPrimitive;

use lib::db;
use lib::diesel::prelude::*;
use lib::diesel::{sql_query, RunQueryDsl};
use lib::service::currency_converter::convert_from;

use crate::db::product::entity::Product;
use crate::db::product_characteristic::product_characteristic_string_value;
use crate::db::product_characteristic::{
    product_characteristic_enum_value, product_characteristic_float_value,
};
use crate::dto::product::{
    CharacteristicEnumValue, CharacteristicFloatValue, CharacteristicIntValue,
    CharacteristicStringValue,
};
use crate::endpoint::product::ProductFilters;

pub fn get_filtered_products(filters: &ProductFilters) -> Vec<Product> {
    let connection = &db::establish_connection();
    let mut query = r"SELECT
    DISTINCT(p.id), p.title, p.description, p.lowest_price, p.highest_price, p.images, p.category, p.enabled, p.created_at, p.updated_at
    FROM product p".to_owned();

    let filter = get_filter_query(&filters);
    let joins = get_join_query(&filters);
    let having = get_having_query(&filters);
    let group_by = get_group_by_query(&filters);

    // TODO group by filters, not by types of query
    query.push_str(&joins);
    query.push_str(&filter);
    query.push_str(&group_by);
    query.push_str(&having);
    query.push_str(" ORDER BY p.id ASC LIMIT 20");
    println!("{}", query);

    sql_query(query)
        .load::<Product>(connection)
        .expect("Error loading products")
}

fn get_group_by_query(filters: &&ProductFilters) -> String {
    let mut group_by = "".to_owned();

    if let Some(chars) = &filters.characteristics {
        // TODO check in not empty chars
        group_by.push_str(" GROUP BY p.id ");
    }

    group_by
}

fn get_having_query(filters: &&ProductFilters) -> String {
    let mut having = "".to_owned();

    if let Some(chars) = &filters.characteristics {
        let mut grouped_filters = vec![];

        for char in &chars.int {
            grouped_filters.push(char.characteristic_id);
        }
        for char in &chars.float {
            grouped_filters.push(char.characteristic_id);
        }
        for char in &chars.enums {
            grouped_filters.push(char.characteristic_id);
        }
        for char in &chars.string {
            grouped_filters.push(char.characteristic_id);
        }

        grouped_filters.sort();
        grouped_filters.dedup();
        let amount_of_unique_chars = grouped_filters.len();
        having.push_str(&format!(
            " HAVING ARRAY_LENGTH(ARRAY_AGG(DISTINCT (c.characteristic_id)), 1) = {} ",
            amount_of_unique_chars
        ));
    }

    having
}

fn get_join_query(filters: &ProductFilters) -> String {
    let mut joins = "".to_owned();

    if filters.source.is_some() {
        joins.push_str(" INNER JOIN source_product sp on p.id = sp.product_id ");
    }

    if let Some(chars) = &filters.characteristics {
        let int_values = get_int_values_expression(&chars.int);
        let float_values = get_float_values_expression(&chars.float);
        let string_values = get_string_values_expression(&chars.string);
        let enum_values = get_enum_values_expression(&chars.enums);
        let mut all_values = vec![];

        if let Some(values) = int_values {
            all_values.push(values);
        }
        if let Some(values) = float_values {
            all_values.push(values);
        }
        if let Some(values) = string_values {
            all_values.push(values);
        }
        if let Some(values) = enum_values {
            all_values.push(values);
        }

        if all_values.is_empty() == false {
            joins.push_str(
                " INNER JOIN product_characteristic c
                    ON (c.product_id = p.id) ",
            );

            let values_str = all_values.join(",");

            joins.push_str(&format!(
                " INNER JOIN (
                       VALUES
                           {}
                   ) f (characteristic_id, value_id)
                   ON (f.characteristic_id = c.characteristic_id AND
                       c.value_id = ANY (f.value_id)) ",
                values_str
            ))
        }
    }

    joins
}

fn get_enum_values_expression(values: &Vec<CharacteristicEnumValue>) -> Option<String> {
    if values.is_empty() {
        return None;
    }
    // TODO no clone
    let enum_value_ids = product_characteristic_enum_value::get_ids_of_values(
        values.iter().map(|v| v.value.clone()).collect(),
    );

    let converted_to_ids = values
        .iter()
        .map(|char| CharacteristicIntValue {
            characteristic_id: char.characteristic_id,
            value: enum_value_ids
                .iter()
                .find(|v| v.value == char.value)
                .expect("Not-existed in db float value")
                .id,
        })
        .collect();

    get_int_values_expression(&converted_to_ids)
}
fn get_string_values_expression(values: &Vec<CharacteristicStringValue>) -> Option<String> {
    if values.is_empty() {
        return None;
    }
    // TODO no clone
    let string_value_ids = product_characteristic_string_value::get_ids_of_values(
        values.iter().map(|v| v.value.clone()).collect(),
    );

    let converted_to_ids = values
        .iter()
        .map(|char| CharacteristicIntValue {
            characteristic_id: char.characteristic_id,
            value: string_value_ids
                .iter()
                .find(|v| v.value == char.value)
                .expect("Not-existed in db float value")
                .id,
        })
        .collect();

    get_int_values_expression(&converted_to_ids)
}
fn get_float_values_expression(values: &Vec<CharacteristicFloatValue>) -> Option<String> {
    if values.is_empty() {
        return None;
    }

    let float_value_ids = product_characteristic_float_value::get_ids_of_values(
        values.iter().map(|v| v.value).collect(),
    );

    let converted_to_ids = values
        .iter()
        .map(|char| CharacteristicIntValue {
            characteristic_id: char.characteristic_id,
            value: float_value_ids
                .iter()
                .find(|v| v.value.to_f32().unwrap() == char.value)
                .expect("Not-existed in db float value")
                .id,
        })
        .collect();

    get_int_values_expression(&converted_to_ids)
}

/// Int values don't have ids and are stored directly as value.
/// So this function can be reused by any other type after some mapping
fn get_int_values_expression(values: &Vec<CharacteristicIntValue>) -> Option<String> {
    if values.is_empty() {
        return None;
    }
    let mut grouped_values: BTreeMap<i16, Vec<String>> = BTreeMap::new();

    for v in values {
        match grouped_values.get_mut(&v.characteristic_id) {
            None => {
                grouped_values.insert(v.characteristic_id, vec![v.value.to_string()]);
            }
            Some(gv) => {
                gv.push(v.value.to_string());
            }
        }
    }

    Some(group_values_to_string(grouped_values))
}

fn group_values_to_string(grouped_values: BTreeMap<i16, Vec<String>>) -> String {
    let v: Vec<String> = grouped_values
        .into_iter()
        .map(|(id, v)| {
            let values = v.join(", ");
            format!("({}, '{{{}}}'::int[])", id, &values)
        })
        .collect();

    v.join(", ")
}

fn get_filter_query(filters: &ProductFilters) -> String {
    let mut filter = "WHERE p.enabled = true".to_owned();

    // TODO params sanitization
    if let Some(filtered_title) = &filters.title {
        let requested_title = filtered_title.to_lowercase();
        filter.push_str(&format!(
            " AND LOWER(p.title) LIKE '%{}%' ",
            requested_title
        ));
    }

    if let Some(filtered_category) = &filters.category {
        let ids: Vec<String> = filtered_category
            .into_iter()
            .map(|v| v.to_string())
            .collect();
        filter.push_str(&format!(" AND p.category IN ({}) ", ids.join(",")));
    }

    if let Some(source) = &filters.source {
        let ids: Vec<String> = source.into_iter().map(|v| v.to_string()).collect();
        filter.push_str(&format!(" AND sp.source_id IN ({}) ", ids.join(",")));
    }

    if let Some(min_price) = filters.min_price {
        filter.push_str(&format!(
            " AND p.highest_price >= {} ",
            convert_from(min_price, filters.currency)
        ));
    }

    if let Some(max_price) = filters.max_price {
        filter.push_str(&format!(
            " AND p.lowest_price <= {} ",
            convert_from(max_price, filters.currency)
        ));
    }

    filter
}

mod tests {
    use crate::db::product::repository::search::get_filter_query;
    use crate::db::product::repository::search::get_int_values_expression;
    use crate::dto::product::CharacteristicIntValue;
    use crate::endpoint::product::ProductFilters;

    #[test]
    fn it_filters_by_title() {
        assert_eq!(
            get_filter_query(&ProductFilters {
                title: Some("title_value".to_owned()),
                ..Default::default()
            }),
            "WHERE p.enabled = true AND LOWER(p.title) LIKE '%title_value%' ".to_owned()
        );
    }

    #[test]
    fn it_filters_by_category() {
        assert_eq!(
            get_filter_query(&ProductFilters {
                category: Some(vec![1, 2]),
                ..Default::default()
            }),
            "WHERE p.enabled = true AND p.category IN (1,2) ".to_owned()
        );
    }

    #[test]
    fn it_filters_by_source() {
        assert_eq!(
            get_filter_query(&ProductFilters {
                source: Some(vec![1, 2]),
                ..Default::default()
            }),
            "WHERE p.enabled = true AND sp.source_id IN (1,2) ".to_owned()
        );
    }

    #[test]
    fn it_creates_int_value_expression() {
        assert_eq!(
            get_int_values_expression(&vec![
                CharacteristicIntValue {
                    characteristic_id: 1,
                    value: 2
                },
                CharacteristicIntValue {
                    characteristic_id: 1,
                    value: 3
                },
                CharacteristicIntValue {
                    characteristic_id: 4,
                    value: 4
                }
            ]),
            "((1, '{2, 3}'::int[]), (4, '{4}'::int[]))".to_owned()
        );
    }
}

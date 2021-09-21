use std::collections::BTreeMap;

use bigdecimal::ToPrimitive;

use lib::db;
use lib::diesel::sql_types::Text;
use lib::diesel::{sql_query, RunQueryDsl};
use lib::service::currency_converter::convert_from;

use crate::db::product::entity::Product;
use crate::db::product_characteristic::product_characteristic_string_value;
use crate::db::product_characteristic::{
    product_characteristic_enum_value, product_characteristic_float_value,
};
use crate::dto::product::{
    CharacteristicEnumValue, CharacteristicFloatValue, CharacteristicIntValue,
    CharacteristicStringValue, ProductCharacteristicsMapped,
};
use crate::endpoint::product::ProductFilters;

pub fn get_filtered_products(filters: &ProductFilters) -> Vec<Product> {
    let connection = &db::establish_connection();
    let mut query = r"SELECT
    DISTINCT(p.id), p.title, p.description, p.lowest_price, p.highest_price, p.images, p.category, p.enabled, p.created_at, p.updated_at
    FROM product p".to_owned();

    let joins = "\n ".to_owned();
    let filter = "\n WHERE p.enabled = true ".to_owned();
    let group_by = "\n ".to_owned();
    let having = "\n ".to_owned();

    let (joins, filter, group_by, having) =
        filter_by_title(joins, filter, group_by, having, &filters.title);
    let (joins, filter, group_by, having) =
        filter_by_category_and_source(joins, filter, group_by, having, filters);
    let (joins, filter, group_by, having) =
        filter_by_price(joins, filter, group_by, having, filters);
    let (joins, filter, group_by, having) =
        filter_by_characteristics(joins, filter, group_by, having, &filters.characteristics);

    query.push_str(&joins);
    query.push_str(&filter);
    query.push_str(&group_by);
    query.push_str(&having);
    query.push_str("\n ORDER BY p.id ASC LIMIT 20");

    if filters.title.is_some() {
        sql_query(query)
            .bind::<Text, _>(format!(
                "%{}%",
                filters.title.as_ref().unwrap().to_lowercase()
            ))
            .load::<Product>(connection)
            .expect("Error loading products")
    } else {
        sql_query(query)
            .load::<Product>(connection)
            .expect("Error loading products")
    }
}

fn filter_by_characteristics(
    mut joins: String,
    filter: String,
    mut group_by: String,
    mut having: String,
    filters: &Option<ProductCharacteristicsMapped>,
) -> (String, String, String, String) {
    if filters.is_none() {
        return (joins, filter, group_by, having);
    }

    let chars = filters.as_ref().unwrap();
    // Int values don't have ids and are stored directly as value.
    let int_values = get_id_values_expression(&chars.int);
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

    if !all_values.is_empty() {
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
        ));

        group_by.push_str(" GROUP BY p.id ");

        having.push_str(&get_having_for_characteristic_filter(chars));
    }

    (joins, filter, group_by, having)
}

fn get_having_for_characteristic_filter(chars: &ProductCharacteristicsMapped) -> String {
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

    grouped_filters.sort_unstable();
    grouped_filters.dedup();
    let amount_of_unique_chars = grouped_filters.len();

    format!(
        " HAVING ARRAY_LENGTH(ARRAY_AGG(DISTINCT (c.characteristic_id)), 1) = {} ",
        amount_of_unique_chars
    )
}

fn filter_by_price(
    joins: String,
    mut filter: String,
    group_by: String,
    having: String,
    filters: &ProductFilters,
) -> (String, String, String, String) {
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

    (joins, filter, group_by, having)
}

fn filter_by_category_and_source(
    mut joins: String,
    mut filter: String,
    group_by: String,
    having: String,
    filters: &ProductFilters,
) -> (String, String, String, String) {
    if let Some(filtered_category) = &filters.category {
        let ids: Vec<String> = filtered_category
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        filter.push_str(&format!(" AND p.category IN ({}) ", ids.join(",")));
    }

    if let Some(source) = &filters.source {
        let ids: Vec<String> = source
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        filter.push_str(&format!(" AND sp.source_id IN ({}) ", ids.join(",")));

        joins.push_str(" INNER JOIN source_product sp on p.id = sp.product_id ");
    }

    (joins, filter, group_by, having)
}

fn filter_by_title(
    joins: String,
    mut filter: String,
    group_by: String,
    having: String,
    title: &Option<String>,
) -> (String, String, String, String) {
    if title.is_some() {
        filter.push_str(" AND LOWER(p.title) LIKE $1 ");
    }

    (joins, filter, group_by, having)
}

fn get_enum_values_expression(values: &[CharacteristicEnumValue]) -> Option<String> {
    if values.is_empty() {
        return None;
    }
    // TODO no clone
    let enum_value_ids = product_characteristic_enum_value::get_ids_of_values(
        values.iter().map(|v| v.value.clone()).collect(),
    );

    let converted_to_ids: Vec<CharacteristicIntValue> = values
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

    get_id_values_expression(&converted_to_ids)
}
fn get_string_values_expression(values: &[CharacteristicStringValue]) -> Option<String> {
    if values.is_empty() {
        return None;
    }
    // TODO no clone
    let string_value_ids = product_characteristic_string_value::get_ids_of_values(
        values.iter().map(|v| v.value.clone()).collect(),
    );

    let converted_to_ids: Vec<CharacteristicIntValue> = values
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

    get_id_values_expression(&converted_to_ids)
}
fn get_float_values_expression(values: &[CharacteristicFloatValue]) -> Option<String> {
    if values.is_empty() {
        return None;
    }

    let float_value_ids = product_characteristic_float_value::get_ids_of_values(
        &values.iter().map(|v| v.value).collect(),
    );

    let converted_to_ids: Vec<CharacteristicIntValue> = values
        .iter()
        .map(|char| CharacteristicIntValue {
            characteristic_id: char.characteristic_id,
            value: float_value_ids
                .iter()
                .find(|v| (v.value.to_f32().unwrap() - char.value).abs() < f32::EPSILON)
                .expect("Not-existed in db float value")
                .id,
        })
        .collect();

    get_id_values_expression(&converted_to_ids)
}

fn get_id_values_expression(values: &[CharacteristicIntValue]) -> Option<String> {
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

mod tests {
    use crate::db::product::repository::search::get_id_values_expression;
    use crate::dto::product::CharacteristicIntValue;

    #[test]
    fn it_creates_int_value_expression() {
        assert_eq!(
            get_id_values_expression(&[CharacteristicIntValue {
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
                }]),
            Some("(1, '{2, 3}'::int[]), (4, '{4}'::int[])".to_owned())
        );
    }
}

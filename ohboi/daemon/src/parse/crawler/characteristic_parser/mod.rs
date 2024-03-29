use lib::error_reporting;
use lib::error_reporting::ReportingContext;

use crate::db::entity::source::SourceName;
pub use crate::parse::crawler::characteristic_parser::bool_value_parser::*;
pub use crate::parse::crawler::characteristic_parser::enum_value_parser::*;
pub use crate::parse::crawler::characteristic_parser::float_value_parser::*;
pub use crate::parse::crawler::characteristic_parser::int_value_parser::*;
pub use crate::parse::crawler::characteristic_parser::string_value_parser::*;
use crate::parse::crawler::Crawler;
use crate::ConsumerName;

mod bool_value_parser;
mod enum_value_parser;
mod float_value_parser;
mod int_value_parser;
mod string_value_parser;

pub fn combine_titles_and_values(
    titles: &[String],
    values: &[String],
) -> Vec<(String, String)> {
    titles
        .iter()
        .enumerate()
        // TODO not clone
        .map(|(k, v)| (v.clone(), values[k].clone()))
        .collect()
}

pub fn parse_and_take<R>(
    characteristics: &mut Vec<(String, String)>,
    crawler: &dyn Crawler,
    external_id: &str,
    predicate: fn(&str, &str, context: &CharacteristicParsingContext) -> Option<R>,
) -> Vec<R> {
    let mut result: Vec<R> = vec![];
    let mut indexes_to_remove = vec![];
    let source = crawler.get_source();

    for (index, (title, value)) in characteristics.iter().enumerate() {
        let context = CharacteristicParsingContext {
            title,
            external_id,
            source,
        };

        if let Some(v) = predicate(title, value, &context) {
            result.push(v);
            indexes_to_remove.push(index);
        }
    }

    // We should delete from end to start
    indexes_to_remove.sort_by(|a, b| b.cmp(a));
    for index in indexes_to_remove {
        characteristics.remove(index);
    }

    result
}

pub fn parse_and_take_multiple<R>(
    characteristics: &mut Vec<(String, String)>,
    crawler: &dyn Crawler,
    external_id: &str,
    predicate: fn(&str, &str, context: &CharacteristicParsingContext) -> Vec<R>,
) -> Vec<R> {
    let mut result: Vec<R> = vec![];
    let mut indexes_to_remove = vec![];
    let source = crawler.get_source();

    for (index, (title, value)) in characteristics.iter().enumerate() {
        let context = CharacteristicParsingContext {
            title,
            external_id,
            source,
        };

        let values = predicate(title, value, &context);
        if !values.is_empty() {
            for v in values {
                result.push(v);
            }
            indexes_to_remove.push(index);
        }
    }

    // We should delete from end to start
    indexes_to_remove.sort_by(|a, b| b.cmp(a));
    for index in indexes_to_remove {
        characteristics.remove(index);
    }

    result
}

type Parser<SomeEnum> = fn(&str) -> Option<SomeEnum>;

pub fn multiple_parse_and_capture<SomeEnum>(
    context: &CharacteristicParsingContext,
    value: &str,
    parser: Parser<SomeEnum>,
) -> Vec<SomeEnum> {
    let parsed_values = value
        .split(',')
        .into_iter()
        .map(|v| parse_and_capture(context, v, parser));

    let mut values = vec![];
    for v in parsed_values.into_iter().flatten() {
        values.push(v)
    }

    values
}

pub fn parse_and_capture<SomeEnum>(
    context: &CharacteristicParsingContext,
    value: &str,
    parser: Parser<SomeEnum>,
) -> Option<SomeEnum> {
    let parsed = parser(value);

    if parsed.is_none() {
        error_reporting::warning(
            format!(
                "[{source}] Can't parse characteristic ({title}) with value ({value}) for [{external_id}]: Unknown value",
                source = context.source,
                title = context.title,
                value = value,
                external_id = context.external_id,
            )
                .as_str(),
            &ReportingContext {
                executor: &ConsumerName::ParseDetails,
                action: "parse_generic"
            }
        );
    }

    parsed
}

pub struct CharacteristicParsingContext<'root> {
    pub title: &'root str,
    pub external_id: &'root str,
    pub source: SourceName,
}

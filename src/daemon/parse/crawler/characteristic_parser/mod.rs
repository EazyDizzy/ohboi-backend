mod bool_value_parser;
mod enum_value_parser;
mod float_value_parser;
mod int_value_parser;
mod string_value_parser;

pub use crate::daemon::parse::crawler::characteristic_parser::bool_value_parser::*;
pub use crate::daemon::parse::crawler::characteristic_parser::enum_value_parser::*;
pub use crate::daemon::parse::crawler::characteristic_parser::float_value_parser::*;
pub use crate::daemon::parse::crawler::characteristic_parser::int_value_parser::*;
pub use crate::daemon::parse::crawler::characteristic_parser::string_value_parser::*;
use crate::daemon::db::entity::source::SourceName;

type Parser<SomeEnum> = fn(&str) -> Option<SomeEnum>;

pub fn multiple_parse_and_capture<SomeEnum>(
    context: &CharacteristicParsingContext,
    value: &str,
    parser: Parser<SomeEnum>,
) -> Vec<SomeEnum> {
    let parsed_values: Vec<Option<SomeEnum>> = value
        .split(",")
        .into_iter()
        .map(|v| parse_and_capture(context, v, parser))
        .collect();

    let mut values = vec![];
    for v in parsed_values {
        if v.is_some() {
            values.push(v.unwrap())
        }
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
        sentry::capture_message(
            format!(
                "[{source}] Can't daemon string characteristic ({title}) with value ({value}) for [{external_id}]: Unknown value",
                source = context.source,
                title = context.title,
                value = value,
                external_id = context.external_id,
            )
                .as_str(),
            sentry::Level::Warning,
        );
    }

    parsed
}

pub struct CharacteristicParsingContext<'root> {
    pub title: &'root str,
    pub external_id: &'root str,
    pub source: SourceName,
}

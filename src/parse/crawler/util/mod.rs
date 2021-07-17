pub use float_value_parser::*;
pub use int_value_parser::*;
pub use string_value_parser::*;

mod float_value_parser;
mod int_value_parser;
mod string_value_parser;

type Parser<SomeEnum> = fn(&str) -> Option<SomeEnum>;

pub fn multiple_parse_and_capture<SomeEnum>(
    title: &str,
    external_id: &str,
    value: &str,
    parser: Parser<SomeEnum>,
) -> Vec<SomeEnum> {
    let parsed_values: Vec<Option<SomeEnum>> = value
        .split(",")
        .into_iter()
        .map(|v| parse_and_capture(title, external_id, v, parser))
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
    title: &str,
    external_id: &str,
    value: &str,
    parser: Parser<SomeEnum>,
) -> Option<SomeEnum> {
    let parsed = parser(value);

    if parsed.is_none() {
        sentry::capture_message(
            format!(
                "Can't parse string characteristic ({title}) with value ({value}) for [{external_id}]: Unknown value",
                title = title,
                value = value,
                external_id = external_id
            )
                .as_str(),
            sentry::Level::Warning,
        );
    }

    parsed
}

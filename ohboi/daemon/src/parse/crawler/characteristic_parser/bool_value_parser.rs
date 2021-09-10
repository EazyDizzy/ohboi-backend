use lib::error_reporting;
use lib::error_reporting::ReportingContext;

use crate::parse::crawler::characteristic_parser::CharacteristicParsingContext;
use crate::ConsumerName;

pub fn bool_value(context: &CharacteristicParsingContext, value: &str) -> Option<bool> {
    match value.trim() {
        "Да" => Some(true),
        "Нет" => Some(false),
        _ => {
            error_reporting::warning(
                format!(
                    "[{source}] Can't parse bool characteristic ({title}) with value ({value}) for [{external_id}]",
                    source = context.source,
                    title = context.title,
                    value = value,
                    external_id = context.external_id,
                )
                    .as_str(),
                &ReportingContext {
                    executor: &ConsumerName::ParseDetails,
                    action: "parse_bool"
                }
            );
            None
        }
    }
}

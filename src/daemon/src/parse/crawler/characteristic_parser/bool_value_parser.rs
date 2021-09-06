use crate::parse::crawler::characteristic_parser::CharacteristicParsingContext;
use lib::local_sentry;

pub fn bool_value(context: &CharacteristicParsingContext, value: &str) -> Option<bool> {
    match value.trim() {
        "Да" => Some(true),
        "Нет" => Some(false),
        _ => {
            local_sentry::capture_message(
                format!(
                    "[{source}] Can't parse bool characteristic ({title}) with value ({value}) for [{external_id}]",
                    source = context.source,
                    title = context.title,
                    value = value,
                    external_id = context.external_id,
                )
                    .as_str(),
                local_sentry::Level::Warning,
            );
            None
        }
    }
}

use crate::daemon::crawler::util::CharacteristicParsingContext;

pub fn bool_value(context: &CharacteristicParsingContext, value: &str) -> Option<bool> {
    match value.trim() {
        "Да" => Some(true),
        "Нет" => Some(false),
        _ => {
            sentry::capture_message(
                format!(
                    "[{source}] Can't parse bool characteristic ({title}) with value ({value}) for [{external_id}]",
                    source = context.source,
                    title = context.title,
                    value = value,
                    external_id = context.external_id,
                )
                    .as_str(),
                sentry::Level::Warning,
            );
            None
        }
    }
}

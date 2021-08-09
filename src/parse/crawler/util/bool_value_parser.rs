pub fn bool_value(title: &str, external_id: &str, value: &str) -> Option<bool> {
    match value.trim() {
        "Да" => Some(true),
        "Нет" => Some(false),
        _ => {
            sentry::capture_message(
                format!(
                    "Can't parse bool characteristic ({title}) with value ({value}) for [{external_id}]",
                    title = title,
                    value = value,
                    external_id = external_id,
                )
                    .as_str(),
                sentry::Level::Warning,
            );
            None
        }
    }
}

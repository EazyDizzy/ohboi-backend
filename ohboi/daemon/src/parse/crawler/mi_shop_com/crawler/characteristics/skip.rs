use crate::parse::crawler::characteristic_parser::CharacteristicParsingContext;

pub fn skip_unneeded_characteristics(
    title: &str,
    value: &str,
    context: CharacteristicParsingContext,
) -> Option<bool> {
    match title {
        "Видеозапись" => Some(true),
        "Сенсорный дисплей" => Some(true),
        "Примечание" => Some(true),
        "Видеоплеер" => Some(true),
        "Аудиоплеер" => Some(true),
        _ => None,
    }
}

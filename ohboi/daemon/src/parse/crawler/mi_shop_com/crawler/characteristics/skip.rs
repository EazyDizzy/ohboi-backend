use crate::parse::crawler::characteristic_parser::CharacteristicParsingContext;

pub fn skip_unneeded_characteristics(
    title: &str,
    _: &str,
    _: &CharacteristicParsingContext,
) -> Option<bool> {
    match title {
        "Видеозапись" | "Сенсорный дисплей" | "Примечание" | "Видеоплеер" | "Аудиоплеер" => {
            Some(true)
        }
        _ => None,
    }
}

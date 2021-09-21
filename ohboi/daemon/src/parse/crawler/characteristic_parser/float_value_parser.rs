use bigdecimal::Num;

use lib::error_reporting;
use lib::error_reporting::ReportingContext;

use crate::parse::crawler::characteristic_parser::CharacteristicParsingContext;
use crate::ConsumerName;

pub fn float_android_version_value(
    context: &CharacteristicParsingContext,
    value: &str,
) -> Option<f32> {
    float_version_value(context, &value.replace("Android", "").replace("OS, v", ""))
}
pub fn float_miui_version_value(
    context: &CharacteristicParsingContext,
    value: &str,
) -> Option<f32> {
    float_version_value(context, &value.replace("MIUI", ""))
}

pub fn float_version_value(context: &CharacteristicParsingContext, mut value: &str) -> Option<f32> {
    let dots: Vec<(usize, &str)> = value.match_indices('.').into_iter().collect();
    if dots.len() > 1 {
        value = &value[0..dots.get(1).unwrap().0];
    }

    float_value(context, value)
}

pub fn float_ghz_value(context: &CharacteristicParsingContext, value: &str) -> Option<f32> {
    let value = value.to_lowercase();
    let was_in_mgz = value.contains("мгц") || value.contains("mhz");

    float_value(
        context,
        value
            .to_lowercase()
            .replace("ггц", "")
            .replace("мгц", "")
            .replace("mhz", "")
            .replace("ghz", "")
            .as_str(),
    )
    .map(|v| if was_in_mgz { v / 1000.0 } else { v })
}
pub fn float_diagonal_value(context: &CharacteristicParsingContext, value: &str) -> Option<f32> {
    float_value(context, value.replace('"', "").as_str())
}

/// `f/1,79 + f/2,4 + f/2,4` -> `1.79`
/// `f2.4` | `ƒ2.4` -> `2.4`
pub fn float_aperture_value(context: &CharacteristicParsingContext, value: &str) -> Option<f32> {
    float_value(
        context,
        value
            .replace("f", "")
            .replace("ƒ", "")
            .replace("/", "")
            .split('+')
            .into_iter()
            .next()
            .unwrap(),
    )
}

pub fn float_value(context: &CharacteristicParsingContext, value: &str) -> Option<f32> {
    match f32::from_str_radix(value.replace(",", ".").trim(), 10) {
        Ok(v) => Some(v),
        Err(e) => {
            error_reporting::warning(
                format!(
                    "[{source}] Can't parse float characteristic ({title}) with value ({value}) for [{external_id}]: {error:?}",
                    source = context.source,
                    title = context.title,
                    value = value,
                    external_id = context.external_id,
                    error = e,
                )
                    .as_str(),
                &ReportingContext {
                    executor: &ConsumerName::ParseDetails,
                    action: "parse_float"
                }
            );
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::crawler::characteristic_parser::{
        float_android_version_value, float_ghz_value, float_miui_version_value, float_value,
        float_version_value, CharacteristicParsingContext, SourceName,
    };

    fn get_context() -> CharacteristicParsingContext<'static> {
        CharacteristicParsingContext {
            title: "_",
            external_id: "_",
            source: SourceName::MiShopCom,
        }
    }

    #[test]
    fn it_parses_android_version() {
        assert_eq!(
            float_android_version_value(&get_context(), "Android 11"),
            Some(11.0)
        );
        assert_eq!(
            float_android_version_value(&get_context(), "OS, v6.2"),
            Some(6.2)
        );
        assert_eq!(
            float_android_version_value(&get_context(), "OS, v6.0.1"),
            Some(6.0)
        );
        assert_eq!(
            float_android_version_value(&get_context(), "OS, v6,2"),
            Some(6.2)
        );
    }

    #[test]
    fn it_parses_miui_version() {
        assert_eq!(
            float_miui_version_value(&get_context(), "MIUI 11"),
            Some(11.0)
        );
        assert_eq!(
            float_miui_version_value(&get_context(), "MIUI 11.2.3"),
            Some(11.2)
        );
    }

    #[test]
    fn it_parses_version() {
        assert_eq!(float_version_value(&get_context(), "7.1.2"), Some(7.1));
        assert_eq!(float_version_value(&get_context(), "0.1.2"), Some(0.1));
    }
    #[test]
    fn it_parses_float() {
        assert_eq!(float_value(&get_context(), "11.2"), Some(11.2));
        assert_eq!(float_value(&get_context(), "11,2"), Some(11.2));
        assert_eq!(float_value(&get_context(), "11,2"), Some(11.2));
    }

    #[test]
    fn it_parses_ghz_float() {
        assert_eq!(float_ghz_value(&get_context(), "2.2ГГц"), Some(2.2));
        assert_eq!(float_ghz_value(&get_context(), "2.2 ГГц"), Some(2.2));
        assert_eq!(float_ghz_value(&get_context(), "2.2Ггц"), Some(2.2));
        assert_eq!(float_ghz_value(&get_context(), "2.2GHz"), Some(2.2));
        assert_eq!(float_ghz_value(&get_context(), "2200 МГц"), Some(2.2));
    }
}

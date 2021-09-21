use lib::error_reporting;
use lib::error_reporting::ReportingContext;

use crate::parse::crawler::characteristic_parser::CharacteristicParsingContext;
use crate::ConsumerName;

/// It skips additional cameras
/// `64Мп + 8Мп + 6Мп` will result in just `64`
pub fn int_mp_value(context: &CharacteristicParsingContext, value: &str) -> Option<i32> {
    int_value(
        context,
        value
            .replace("Мп", "")
            .split("+")
            .into_iter()
            .next()
            .unwrap(),
    )
}
pub fn int_ma_h_value(context: &CharacteristicParsingContext, value: &str) -> Option<i32> {
    int_value(context, &value.replace("мАч", ""))
}
pub fn int_nit_value(context: &CharacteristicParsingContext, value: &str) -> Option<i32> {
    int_value(context, &value.replace("нит", ""))
}
pub fn int_max_memory_card_size_value(
    context: &CharacteristicParsingContext,
    value: &str,
) -> Option<i32> {
    int_value(context, &value.replace("до", "").replace("ГБ", ""))
}

pub fn int_guarantee_value(context: &CharacteristicParsingContext, value: &str) -> Option<i32> {
    int_value(
        context,
        &value.replace("месяцев", "").replace("Месяцев.", ""),
    )
}
pub fn int_hz_value(context: &CharacteristicParsingContext, value: &str) -> Option<i32> {
    int_value(context, &value.replace("Гц", ""))
}
pub fn int_memory_value(context: &CharacteristicParsingContext, value: &str) -> Option<i32> {
    let is_tb = value.contains("ТБ");
    int_value(context, &value.replace("ГБ", "").replace("до", "")).and_then(|v| {
        if is_tb {
            Some(v * 1000)
        } else {
            Some(v)
        }
    })
}
pub fn int_fps_value(context: &CharacteristicParsingContext, value: &str) -> Option<i32> {
    int_value(context, &value.replace("fps", "").replace("кадров/с", ""))
}
/// `4K` -> `4000`
/// `720px` | `720p` -> `720`
/// `1920x1080` -> `1920`
/// `1080/720` -> `1080`
pub fn pix_int_value(context: &CharacteristicParsingContext, value: &str) -> Option<i32> {
    let mut cut_value = value;

    if cut_value.contains('x') {
        cut_value = cut_value.split('x').into_iter().next().unwrap();
    }
    if cut_value.contains('/') {
        cut_value = cut_value.split('/').into_iter().next().unwrap();
    }

    int_value(
        context,
        &cut_value
            .replace("K", "000")
            .replace("К", "000")
            .replace("px", "")
            .replace("p", ""),
    )
}

pub fn multiple_int_value(context: &CharacteristicParsingContext, value: &str) -> Vec<i32> {
    let parsed_values: Vec<Option<i32>> = value
        .split(',')
        .into_iter()
        .map(|v| int_value(context, v))
        .collect();

    let mut int_values = vec![];
    for v in parsed_values {
        if v.is_some() {
            int_values.push(v.unwrap())
        }
    }

    int_values
}
pub fn int_value(context: &CharacteristicParsingContext, value: &str) -> Option<i32> {
    match value.trim().parse::<i32>() {
        Ok(v) => Some(v),
        Err(e) => {
            error_reporting::warning(
                format!(
                    "[{source}] Can't parse int characteristic ({title}) with value ({value}) for [{external_id}]: {error:?}",
                    source = context.source,
                    title = context.title,
                    value = value,
                    external_id = context.external_id,
                    error = e,
                )
                    .as_str(),
                &ReportingContext {
                    executor: &ConsumerName::ParseDetails,
                    action: "parse_int"
                }
            );
            None
        }
    }
}

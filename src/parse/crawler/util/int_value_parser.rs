
/// It skips additional cameras
/// `64Мп + 8Мп + 6Мп` will result in just `64`
pub fn int_mp_value(title: &str, external_id: &str, value: &str) -> Option<i32> {
    int_value(
        title,
        external_id,
        &value
            .replace("Мп", "")
            .split("+")
            .into_iter()
            .next()
            .unwrap()
            .trim(),
    )
}
pub fn int_ma_h_value(title: &str, external_id: &str, value: &str) -> Option<i32> {
    int_value(title, external_id, &value.replace("мАч", "").trim())
}
pub fn int_hz_value(title: &str, external_id: &str, value: &str) -> Option<i32> {
    int_value(title, external_id, &value.replace("Гц", "").trim())
}
pub fn int_fps_value(title: &str, external_id: &str, value: &str) -> Option<i32> {
    int_value(title, external_id, &value.replace("fps", "").trim())
}
pub fn pix_int_value(title: &str, external_id: &str, value: &str) -> Option<i32> {
    int_value(
        title,
        external_id,
        &value.replace("K", "000").replace("К", "000"),
    )
}

pub fn multiple_int_value(title: &str, external_id: &str, value: &str) -> Vec<i32> {
    let parsed_values: Vec<Option<i32>> = value
        .split(",")
        .into_iter()
        .map(|v| int_value(title, external_id, v))
        .collect();

    let mut int_values = vec![];
    for v in parsed_values {
        if v.is_some() {
            int_values.push(v.unwrap())
        }
    }

    int_values
}
pub fn int_value(title: &str, external_id: &str, value: &str) -> Option<i32> {
    match i32::from_str_radix(value.trim(), 10) {
        Ok(v) => Some(v),
        Err(e) => {
            sentry::capture_message(
                format!(
                    "Can't parse int characteristic ({title}) with value ({value}) for [{external_id}]: {error:?}",
                    title = title,
                    value = value,
                    external_id = external_id,
                    error = e,
                )
                    .as_str(),
                sentry::Level::Warning,
            );
            None
        }
    }
}
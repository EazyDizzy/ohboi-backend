use bigdecimal::Num;

pub fn float_android_version_value(title: &str, external_id: &str, value: &str) -> Option<f32> {
    float_value(title, external_id, &value.replace("Android", "").trim())
}
pub fn float_ghz_value(title: &str, external_id: &str, value: &str) -> Option<f32> {
    float_value(title, external_id, value.replace("ГГц", "").trim())
}
pub fn float_diagonal_value(title: &str, external_id: &str, value: &str) -> Option<f32> {
    float_value(title, external_id, value.replace('"', "").trim())
}

/// It skips additional apertures
/// `f/1,79 + f/2,4 + f/2,4` will result in just `1.79`
pub fn float_aperture_value(title: &str, external_id: &str, value: &str) -> Option<f32> {
    float_value(
        title,
        external_id,
        &value
            .replace("f/", "")
            .split("+")
            .into_iter()
            .next()
            .unwrap()
            .trim(),
    )
}

pub fn float_value(title: &str, external_id: &str, value: &str) -> Option<f32> {
    match f32::from_str_radix(value.replace(",", ".").as_str(), 10) {
        Ok(v) => Some(v),
        Err(e) => {
            sentry::capture_message(
                format!(
                    "Can't parse float characteristic ({title}) with value ({value}) for [{external_id}]: {error:?}",
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

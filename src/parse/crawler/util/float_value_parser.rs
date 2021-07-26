use bigdecimal::Num;

pub fn float_android_version_value(title: &str, external_id: &str, value: &str) -> Option<f32> {
    float_version_value(
        title,
        external_id,
        &value.replace("Android", "").replace("OS, v", ""),
    )
}
pub fn float_miui_version_value(title: &str, external_id: &str, value: &str) -> Option<f32> {
    float_version_value(title, external_id, &value.replace("MIUI", ""))
}

pub fn float_version_value(title: &str, external_id: &str, mut value: &str) -> Option<f32> {
    let dots: Vec<(usize, &str)> = value.match_indices(".").into_iter().collect();
    if dots.len() > 1 {
        value = &value[0..dots.get(1).unwrap().0];
    }

    float_value(title, external_id, value)
}

pub fn float_ghz_value(title: &str, external_id: &str, value: &str) -> Option<f32> {
    let value = value.to_lowercase();
    let was_in_mgz = value.contains("мгц") || value.contains("mhz");

    float_value(
        title,
        external_id,
        value
            .to_lowercase()
            .replace("ггц", "")
            .replace("мгц", "")
            .replace("mhz", "")
            .replace("ghz", "")
            .as_str(),
    )
    .map_or(None, |v| {
        if was_in_mgz {
            Some(v / 1000.0)
        } else {
            Some(v)
        }
    })
}
pub fn float_diagonal_value(title: &str, external_id: &str, value: &str) -> Option<f32> {
    float_value(title, external_id, value.replace('"', "").as_str())
}

/// `f/1,79 + f/2,4 + f/2,4` -> `1.79`
/// `f2.4` | `ƒ2.4` -> `2.4`
pub fn float_aperture_value(title: &str, external_id: &str, value: &str) -> Option<f32> {
    float_value(
        title,
        external_id,
        &value
            .replace("f", "")
            .replace("ƒ", "")
            .replace("/", "")
            .split("+")
            .into_iter()
            .next()
            .unwrap(),
    )
}

pub fn float_value(title: &str, external_id: &str, value: &str) -> Option<f32> {
    match f32::from_str_radix(value.replace(",", ".").trim(), 10) {
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

#[cfg(test)]
mod tests {
    use crate::parse::crawler::util::*;

    #[test]
    fn it_parses_android_version() {
        assert_eq!(
            float_android_version_value("_", "_", "Android 11"),
            Some(11.0)
        );
        assert_eq!(float_android_version_value("_", "_", "OS, v6.2"), Some(6.2));
        assert_eq!(
            float_android_version_value("_", "_", "OS, v6.0.1"),
            Some(6.0)
        );
        assert_eq!(float_android_version_value("_", "_", "OS, v6,2"), Some(6.2));
    }

    #[test]
    fn it_parses_miui_version() {
        assert_eq!(float_miui_version_value("_", "_", "MIUI 11"), Some(11.0));
        assert_eq!(
            float_miui_version_value("_", "_", "MIUI 11.2.3"),
            Some(11.2)
        );
    }

    #[test]
    fn it_parses_version() {
        assert_eq!(float_version_value("_", "_", "7.1.2"), Some(7.1));
        assert_eq!(float_version_value("_", "_", "0.1.2"), Some(0.1));
    }
    #[test]
    fn it_parses_float() {
        assert_eq!(float_value("_", "_", "11.2"), Some(11.2));
        assert_eq!(float_value("_", "_", "11,2"), Some(11.2));
        assert_eq!(float_value("_", "_", "11,2"), Some(11.2));
    }

    #[test]
    fn it_parses_ghz_float() {
        assert_eq!(float_ghz_value("_", "_", "2.2ГГц"), Some(2.2));
        assert_eq!(float_ghz_value("_", "_", "2.2 ГГц"), Some(2.2));
        assert_eq!(float_ghz_value("_", "_", "2.2Ггц"), Some(2.2));
        assert_eq!(float_ghz_value("_", "_", "2.2GHz"), Some(2.2));
        assert_eq!(float_ghz_value("_", "_", "2200 МГц"), Some(2.2));
    }
}

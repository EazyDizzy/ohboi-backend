use maplit::btreemap;
use regex::Regex;

use lib::dto::characteristic::enum_characteristic::MediaFormat;
use crate::daemon::parse::crawler::characteristic_parser::{
    enum_media_format_value, string_value, CharacteristicParsingContext,
};

lazy_static! {
    static ref NO_DESCRIPTION_RE: Regex = Regex::new(r"(?ms)[A-Za-z./ 0-9\-+–]{2,}").unwrap();
}
pub fn multiple_string_media_format_value(
    context: &CharacteristicParsingContext,
    mut value: &str,
) -> Vec<MediaFormat> {
    let mut formats: Vec<MediaFormat> = vec![];
    let mut values: Vec<&str>;
    if value.ends_with(".") {
        value = &value[0..value.len() - 1]
    }

    if value.contains(";") {
        values = value.split(";").collect();
    } else {
        values = value.split(",").collect();
    }

    let mut index_bonus = 0;
    let mut values_copy = values.clone();
    for (i, v) in values.into_iter().enumerate() {
        let mut additional_values: Vec<&str> = vec![];
        if v.contains("/") {
            additional_values = v.split("/").collect();
        }
        if v.contains(" and ") {
            additional_values = v.split("and").collect();
        }
        if v.contains(" и ") {
            additional_values = v.split("и").collect();
        }

        if !additional_values.is_empty() {
            values_copy.remove(i + index_bonus);

            for add_v in additional_values {
                index_bonus += 1;
                values_copy.insert(i + index_bonus - 1, add_v.trim());
            }

            index_bonus -= 1;
        }
    }
    values = values_copy;
    values = values
        .iter()
        .map(|v| {
            let parsed = NO_DESCRIPTION_RE.captures_iter(v).next();
            match parsed {
                None => None,
                Some(e) => Some(e.get(0).unwrap().as_str()),
            }
        })
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .collect::<Vec<&str>>()
        .to_vec();

    for v in values {
        let mapped = enum_media_format_value(v);
        let exceptions = btreemap! {
            "AAC+eAAC+" => vec![MediaFormat::AAC_plus, MediaFormat::eAAC_plus],
            "H.264 HEVC" => vec![MediaFormat::H264, MediaFormat::H265],
            "ASP" => vec![],
            "/ ASP" => vec![],
            "." => vec![],
        };
        if let Some(format) = mapped {
            if !formats.contains(&format) {
                formats.push(format);
            }
        } else if let Some(exception_values) = exceptions.get(string_value(v).as_str()) {
            for exception_format in exception_values {
                if !formats.contains(&exception_format) {
                    formats.push(*exception_format);
                }
            }
        } else {
            sentry::capture_message(
                format!(
                    "[{source}] Can't parse media format characteristic ({title}) with value ({value}) for [{external_id}]: Unknown value",
                    source = context.source,
                    title = context.title,
                    value = v,
                    external_id = context.external_id
                )
                    .as_str(),
                sentry::Level::Warning,
            );
        }
    }

    formats
}

#[cfg(test)]
mod tests {
    use lib::dto::characteristic::enum_characteristic::MediaFormat;
    use crate::daemon::db::entity::source::SourceName;
    use crate::daemon::parse::crawler::characteristic_parser::CharacteristicParsingContext;
    use crate::daemon::parse::crawler::mi_shop_com::crawler::media_format_parser::multiple_string_media_format_value;

    fn get_context() -> CharacteristicParsingContext<'static> {
        CharacteristicParsingContext {
            title: "_",
            external_id: "_",
            source: SourceName::MiShopCom,
        }
    }

    #[test]
    fn it_parses_media_format() {
        assert_eq!(
            multiple_string_media_format_value(
                &get_context(),
                "MP4; M4V; MKV;XVID; WAV; AAC; MP3; AMR; FLAC; APE",
            ),
            vec![
                MediaFormat::MP4,
                MediaFormat::M4V,
                MediaFormat::MKV,
                MediaFormat::XVID,
                MediaFormat::WAV,
                MediaFormat::AAC,
                MediaFormat::MP3,
                MediaFormat::AMR,
                MediaFormat::FLAC,
                MediaFormat::APE,
            ]
        );

        assert_eq!(
            multiple_string_media_format_value(
                &get_context(),
                "MP4; M4V; MKV; XVID; WAV; AAC/AAC+/eAAC+; MP3; AMR-NB/WB; FLAC; PCM",
            ),
            vec![
                MediaFormat::MP4,
                MediaFormat::M4V,
                MediaFormat::MKV,
                MediaFormat::XVID,
                MediaFormat::WAV,
                MediaFormat::AAC,
                MediaFormat::AAC_plus,
                MediaFormat::eAAC_plus,
                MediaFormat::MP3,
                MediaFormat::AMR_NB,
                MediaFormat::WB,
                MediaFormat::FLAC,
                MediaFormat::PCM,
            ]
        );
        assert_eq!(
            multiple_string_media_format_value(
                &get_context(),
                "PCM, AAC / AAC+, MP3, AMR–NB and WB, Opus, PCM/WAVE",
            ),
            vec![
                MediaFormat::PCM,
                MediaFormat::AAC,
                MediaFormat::AAC_plus,
                MediaFormat::MP3,
                MediaFormat::AMR_NB,
                MediaFormat::WB,
                MediaFormat::Opus,
                MediaFormat::WAVE,
            ]
        );
        assert_eq!(
            multiple_string_media_format_value(&get_context(), "PCM, PCM/WAVE"),
            vec![MediaFormat::PCM, MediaFormat::WAVE,]
        );
        assert_eq!(
            multiple_string_media_format_value(&get_context(), "Поддерживает H.263, H264 (базовый профиль / основной профиль), H.264 HEVC, MPEG4 (простой профиль / ASP), XVID, ASF / WMV, 3GI, MKV / WEBM, M4V, FLV и другие видеоформаты.Поддерживает аудиоформаты, такие как AAC / AAC +, MP3, AMR - NB и WB, FLAC, MIDI / PCM / WAVE"),
            vec![MediaFormat::H263, MediaFormat::H264, MediaFormat::H265, MediaFormat::MPEG4, MediaFormat::XVID, MediaFormat::ASF, MediaFormat::WMV, MediaFormat::_3GI, MediaFormat::MKV, MediaFormat::WEBM, MediaFormat::M4V, MediaFormat::FLV, MediaFormat::AAC, MediaFormat::AAC_plus, MediaFormat::MP3, MediaFormat::AMR_NB, MediaFormat::WB, MediaFormat::FLAC, MediaFormat::MIDI, MediaFormat::PCM, MediaFormat::WAVE]
        );
        assert_eq!(
            multiple_string_media_format_value(&get_context(), "Видео форматы: H.265 / HEVC (основной профиль), H.264 (базовый/основной/высокий), MPEG4 (обычный/ASP) и другие. Аудио форматы: PCM, AAC / AAC + / eAAC +, MP3, AMR - NB и WB, FLAC, WAV."),
            vec![MediaFormat::H265, MediaFormat::H264,MediaFormat::MPEG4,MediaFormat::PCM,MediaFormat::AAC,MediaFormat::AAC_plus,MediaFormat::eAAC_plus,MediaFormat::MP3,MediaFormat::AMR_NB,MediaFormat::WB,MediaFormat::FLAC,MediaFormat::WAV]
        );
    }
}

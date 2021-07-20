use crate::parse::dto::characteristic::string_characteristic::*;

pub fn string_media_format_value(value: &str) -> Option<MediaFormat> {
    match string_value(value).as_str() {
        "MP4" => Some(MediaFormat::MP4),
        "M4V" => Some(MediaFormat::M4V),
        "MKV" => Some(MediaFormat::MKV),
        "XVID" => Some(MediaFormat::XVID),
        "WAV" => Some(MediaFormat::WAV),
        "AAC" => Some(MediaFormat::AAC),
        "MP3" => Some(MediaFormat::MP3),
        "AMR" => Some(MediaFormat::AMR),
        "FLAC" => Some(MediaFormat::FLAC),
        "APE" => Some(MediaFormat::APE),
        "AAC+" | "AAC +" => Some(MediaFormat::AAC_plus),
        "eAAC+" | "eAAC +" => Some(MediaFormat::eAAC_plus),
        "AMR-NB" | "AMR - NB" => Some(MediaFormat::AMR_NB),
        "WB" => Some(MediaFormat::WB),
        "PCM" => Some(MediaFormat::PCM),
        "H.263" | "H263" => Some(MediaFormat::H263),
        "H.264" | "H264" => Some(MediaFormat::H264),
        "H.265" | "HEVC" => Some(MediaFormat::H265),
        "MPEG4" => Some(MediaFormat::MPEG4),
        "ASF" => Some(MediaFormat::ASF),
        "WMV" => Some(MediaFormat::WMV),
        "3GI" => Some(MediaFormat::_3GI),
        "WEBM" => Some(MediaFormat::WEBM),
        "FLV" => Some(MediaFormat::FLV),
        "MIDI" => Some(MediaFormat::MIDI),
        "WAVE" => Some(MediaFormat::WAVE),
        "Opus" => Some(MediaFormat::Opus),
        "VC-1" => Some(MediaFormat::VC1),
        "AMR–NB" => Some(MediaFormat::AMR_NB),
        "DSF" => Some(MediaFormat::DSF),
        "M4A" => Some(MediaFormat::M4A),
        "OGG" => Some(MediaFormat::OGG),
        "WMA" => Some(MediaFormat::WMA),
        "AWB" => Some(MediaFormat::AWB),
        e => {
            println!("unknown '{}'", e);
            None
        }
    }
}
pub fn string_internet_connection_technology_value(
    value: &str,
) -> Option<InternetConnectionTechnology> {
    match string_value(value).as_str() {
        "GPRS" => Some(InternetConnectionTechnology::GPRS),
        "EDGE" => Some(InternetConnectionTechnology::EDGE),
        "3G" => Some(InternetConnectionTechnology::_3G),
        "4G" => Some(InternetConnectionTechnology::_4G),
        "5G" => Some(InternetConnectionTechnology::_5G),
        _ => None,
    }
}
pub fn string_satellite_navigation_value(value: &str) -> Option<SatelliteNavigation> {
    match string_value(value).as_str() {
        "GPS" => Some(SatelliteNavigation::GPS),
        "A-GPS" => Some(SatelliteNavigation::A_GPS),
        "ГЛОНАСС" => Some(SatelliteNavigation::GLONASS),
        "Galileo" => Some(SatelliteNavigation::Galileo),
        "BeiDou" => Some(SatelliteNavigation::BeiDou),
        _ => None,
    }
}
pub fn string_wifi_standard_value(value: &str) -> Option<WifiStandard> {
    match string_value(value).as_str() {
        "n" => Some(WifiStandard::_4),
        "ac" => Some(WifiStandard::_5),
        "ax" => Some(WifiStandard::_6),
        "be" => Some(WifiStandard::_7),
        "a" => Some(WifiStandard::A),
        "b" => Some(WifiStandard::B),
        "g" => Some(WifiStandard::G),
        "gc" => Some(WifiStandard::GC),
        _ => None,
    }
}
pub fn string_material_value(value: &str) -> Option<Material> {
    match string_value(value).as_str() {
        "Металл" => Some(Material::Metal),
        "Стекло" => Some(Material::Glass),
        "Пластик" => Some(Material::Plastic),
        "Алюминий" => Some(Material::Aluminum),
        "Керамика" => Some(Material::Ceramics),
        _ => None,
    }
}
pub fn string_sim_card_value(value: &str) -> Option<SimCard> {
    match string_value(value).as_str() {
        "nano-SIM" => Some(SimCard::Nano),
        "micro-SIM" => Some(SimCard::Micro),
        "mini-SIM" => Some(SimCard::Mini),
        _ => None,
    }
}
pub fn string_charging_connector_type_value(value: &str) -> Option<ChargingConnectorType> {
    match string_value(value).as_str() {
        "USB Type-C" => Some(ChargingConnectorType::USBTypeC),
        "Micro-USB" => Some(ChargingConnectorType::MicroUSB),
        _ => None,
    }
}
pub fn string_country_value(value: &str) -> Option<Country> {
    match string_value(value).as_str() {
        "Китай" => Some(Country::China),
        _ => None,
    }
}
pub fn string_memory_card_slot_value(value: &str) -> Option<MemoryCardSlot> {
    match string_value(value).as_str() {
        "Гибридный" => Some(MemoryCardSlot::Hybrid),
        "Отдельный" => Some(MemoryCardSlot::Separate),
        "Отсутствует" => Some(MemoryCardSlot::None),
        _ => None,
    }
}
pub fn string_audio_jack_value(value: &str) -> Option<AudioJack> {
    match string_value(value).as_str() {
        "3.5" => Some(AudioJack::_3_5mm),
        "USB Type-C" => Some(AudioJack::USBTypeC),
        _ => None,
    }
}
pub fn string_battery_type_value(value: &str) -> Option<BatteryType> {
    match string_value(value).as_str() {
        "Литий-полимерный" => Some(BatteryType::LithiumPolymer),
        "Литий-ионный" => Some(BatteryType::LithiumIon),
        _ => None,
    }
}
pub fn string_display_type_value(value: &str) -> Option<DisplayType> {
    match string_value(value).as_str() {
        "AMOLED" => Some(DisplayType::Amoled),
        "OLED" => Some(DisplayType::Oled),
        "IPS" => Some(DisplayType::IPS),
        _ => None,
    }
}

pub fn string_value(value: &str) -> String {
    value.trim().to_string()
}

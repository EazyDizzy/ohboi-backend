use crate::parse::dto::characteristic::string_characteristic::{AudioJack, BatteryType, ChargingConnectorType, DisplayType, InternetConnectionTechnology, SatelliteNavigation, SimCard, WifiStandard, Country};

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
        _ => None,
    }
}
pub fn string_sim_card_value(value: &str) -> Option<SimCard> {
    match string_value(value).as_str() {
        "nano-SIM" => Some(SimCard::Nano),
        _ => None,
    }
}
pub fn string_charging_connector_type_value(value: &str) -> Option<ChargingConnectorType> {
    match string_value(value).as_str() {
        "USB Type-C" => Some(ChargingConnectorType::USBTypeC),
        _ => None,
    }
}pub fn string_country_value(value: &str) -> Option<Country> {
    match string_value(value).as_str() {
        "Китай" => Some(Country::China),
        _ => None,
    }
}
pub fn string_audio_jack_value(value: &str) -> Option<AudioJack> {
    match string_value(value).as_str() {
        "3.5 мм" => Some(AudioJack::_3_5mm),
        _ => None,
    }
}
pub fn string_battery_type_value(value: &str) -> Option<BatteryType> {
    match string_value(value).as_str() {
        "Литий-полимерный" => Some(BatteryType::LithiumPolymer),
        _ => None,
    }
}
pub fn string_display_type_value(value: &str) -> Option<DisplayType> {
    match string_value(value).as_str() {
        "AMOLED" => Some(DisplayType::Amoled),
        "IPS" => Some(DisplayType::IPS),
        _ => None,
    }
}

pub fn string_value(value: &str) -> String {
    value.trim().to_string()
}

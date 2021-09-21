#![allow(non_camel_case_types)]

use std::fmt;

use strum_macros::EnumVariantNames;

#[derive(Debug, EnumVariantNames)]
pub enum EnumCharacteristic {
    ChargingConnectorType(ChargingConnectorType),
    BatteryType(BatteryType),
    SimCard(SimCard),
    Material(Material),
    DisplayType(DisplayType),
    InternetConnectionTechnology(InternetConnectionTechnology),
    SatelliteNavigation(SatelliteNavigation),
    WifiStandard(WifiStandard),
    AudioJack(AudioJack),
    TechnologySupport(Technology),
    ProducingCountry(Country),
    MemoryCardSlot(MemoryCardSlot),
    SupportedMediaFormat(MediaFormat),
}

impl fmt::Display for EnumCharacteristic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl EnumCharacteristic {
    pub fn full_name(&self) -> String {
        let name = self.name();
        let value = self.value();

        [name, ".".to_string(), value].concat()
    }

    pub fn name(&self) -> String {
        let name = self.to_string();

        name[0..name.find('(').unwrap()].to_string()
    }

    pub fn value(&self) -> String {
        use EnumCharacteristic::*;

        match self {
            ChargingConnectorType(e) => format!("{:?}", e),
            BatteryType(e) => format!("{:?}", e),
            SimCard(e) => format!("{:?}", e),
            Material(e) => format!("{:?}", e),
            DisplayType(e) => format!("{:?}", e),
            InternetConnectionTechnology(e) => format!("{:?}", e),
            SatelliteNavigation(e) => format!("{:?}", e),
            WifiStandard(e) => format!("{:?}", e),
            AudioJack(e) => format!("{:?}", e),
            TechnologySupport(e) => format!("{:?}", e),
            ProducingCountry(e) => format!("{:?}", e),
            MemoryCardSlot(e) => format!("{:?}", e),
            SupportedMediaFormat(e) => format!("{:?}", e),
        }
    }

    pub fn type_from_name(name: &str) -> EnumCharacteristic {
        match name {
            "ChargingConnectorType" => {
                EnumCharacteristic::ChargingConnectorType(ChargingConnectorType::MicroUSB)
            }
            "BatteryType" => EnumCharacteristic::BatteryType(BatteryType::LithiumIon),
            "SimCard" => EnumCharacteristic::SimCard(SimCard::Embedded),
            "Material" => EnumCharacteristic::Material(Material::Aluminum),
            "DisplayType" => EnumCharacteristic::DisplayType(DisplayType::Amoled),
            "InternetConnectionTechnology" => {
                EnumCharacteristic::InternetConnectionTechnology(InternetConnectionTechnology::_3G)
            }
            "SatelliteNavigation" => {
                EnumCharacteristic::SatelliteNavigation(SatelliteNavigation::A_GPS)
            }
            "WifiStandard" => EnumCharacteristic::WifiStandard(WifiStandard::_4),
            "AudioJack" => EnumCharacteristic::AudioJack(AudioJack::_3_5mm),
            "TechnologySupport" => EnumCharacteristic::TechnologySupport(Technology::Autofocus),
            "ProducingCountry" => EnumCharacteristic::ProducingCountry(Country::China),
            "MemoryCardSlot" => EnumCharacteristic::MemoryCardSlot(MemoryCardSlot::Hybrid),
            "SupportedMediaFormat" => EnumCharacteristic::SupportedMediaFormat(MediaFormat::_3GI),
            en => {
                panic!("Unknown enum type {}", en)
            }
        }
    }
}

#[derive(Debug, EnumVariantNames)]
pub enum BatteryType {
    LithiumIon,
    LithiumPolymer,
}

#[derive(Debug, EnumVariantNames)]
pub enum Material {
    Metal,
    Glass,
    Plastic,
    Aluminum,
    Ceramics,
}

#[derive(Debug, EnumVariantNames)]
pub enum SimCard {
    FullSize,
    Mini,
    Micro,
    Nano,
    Embedded,
}
#[derive(Debug, EnumVariantNames)]
pub enum DisplayType {
    Oled,
    Amoled,
    IPS,
}

#[derive(Debug, EnumVariantNames)]
pub enum InternetConnectionTechnology {
    GPRS,
    EDGE,
    _3G,
    _4G,
    _5G,
}

#[derive(Debug, EnumVariantNames)]
pub enum SatelliteNavigation {
    GPS,
    A_GPS,
    Galileo,
    BeiDou,
    GLONASS,
}

#[derive(Debug, EnumVariantNames)]
pub enum WifiStandard {
    _4,
    _5,
    _6,
    _7,
    A,
    B,
    G,
    GC,
}

#[derive(Debug, EnumVariantNames)]
pub enum ChargingConnectorType {
    USBTypeC,
    MicroUSB,
}
#[derive(Debug, EnumVariantNames)]
pub enum AudioJack {
    _3_5mm,
    USBTypeC,
}

#[derive(Debug, EnumVariantNames)]
pub enum Technology {
    NFC,
    FastCharging,
    InfraredPort,
    WirelessCharger,
    Autofocus,
}

#[derive(Debug, EnumVariantNames)]
pub enum Country {
    China,
}

#[derive(Debug, EnumVariantNames)]
pub enum MemoryCardSlot {
    Hybrid,
    Separate,
    None,
}
#[derive(Debug, PartialEq, Copy, Clone, EnumVariantNames)]
pub enum MediaFormat {
    MP4,
    M4V,
    MKV,
    XVID,
    WAV,
    AAC,
    MP3,
    AMR,
    FLAC,
    APE,
    AAC_plus,
    eAAC_plus,
    AMR_NB,
    WB,
    VC1,
    PCM,
    H263,
    H264,
    H265,
    MPEG4,
    ASF,
    WMV,
    _3GI,
    WEBM,
    FLV,
    MIDI,
    WAVE,
    Opus,
    DSF,
    M4A,
    OGG,
    WMA,
    AWB,
}

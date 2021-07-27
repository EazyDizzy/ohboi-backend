use std::fmt;

use serde::Serialize;
use strum_macros::EnumIter;
use strum_macros::EnumVariantNames;

#[derive(Serialize, Debug, PartialEq, Clone, EnumVariantNames)]
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
    pub fn name(&self) -> String {
        let name = self.to_string();

        name[0..name.find("(").unwrap()].to_string()
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
}

#[derive(Serialize, Debug, PartialEq, Copy, Clone, EnumVariantNames)]
pub enum BatteryType {
    LithiumIon,
    LithiumPolymer,
}

#[derive(Serialize, Debug, PartialEq, Copy, Clone, EnumVariantNames)]
pub enum Material {
    Metal,
    Glass,
    Plastic,
    Aluminum,
    Ceramics,
}

#[derive(Serialize, Debug, PartialEq, Copy, Clone, EnumVariantNames)]
pub enum SimCard {
    FullSize,
    Mini,
    Micro,
    Nano,
    Embedded,
}
#[derive(Serialize, Debug, PartialEq, Copy, Clone, EnumVariantNames)]
pub enum DisplayType {
    Oled,
    Amoled,
    IPS,
}

#[derive(Serialize, Debug, PartialEq, Copy, Clone, EnumVariantNames)]
pub enum InternetConnectionTechnology {
    GPRS,
    EDGE,
    _3G,
    _4G,
    _5G,
}

#[derive(Serialize, Debug, PartialEq, Copy, Clone, EnumVariantNames)]
pub enum SatelliteNavigation {
    GPS,
    A_GPS,
    Galileo,
    BeiDou,
    GLONASS,
}

#[derive(Serialize, Debug, PartialEq, Copy, Clone, EnumVariantNames)]
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

#[derive(Serialize, Debug, PartialEq, Copy, Clone, EnumVariantNames)]
pub enum ChargingConnectorType {
    USBTypeC,
    MicroUSB,
}
#[derive(Serialize, Debug, PartialEq, Copy, Clone, EnumVariantNames)]
pub enum AudioJack {
    _3_5mm,
    USBTypeC,
}

#[derive(Serialize, Debug, PartialEq, Copy, Clone, EnumVariantNames)]
pub enum Technology {
    NFC,
    FastCharging,
    InfraredPort,
    WirelessCharger,
    Autofocus,
}

#[derive(Serialize, Debug, PartialEq, Copy, Clone, EnumVariantNames)]
pub enum Country {
    China,
}

#[derive(Serialize, Debug, PartialEq, Copy, Clone, EnumVariantNames)]
pub enum MemoryCardSlot {
    Hybrid,
    Separate,
    None,
}
#[derive(Serialize, Debug, PartialEq, Copy, Clone, EnumVariantNames)]
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

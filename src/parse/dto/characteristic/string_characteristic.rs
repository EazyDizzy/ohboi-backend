use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub enum StringCharacteristic {
    Processor(String),
    VideoProcessor(String),
    AspectRatio(String),
    DisplayResolution(String),
    Contrast(String),
    //
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
    SupportedMediaFormats(MediaFormat),
}

#[derive(Serialize, Debug, PartialEq)]
pub enum BatteryType {
    LithiumIon,
    LithiumPolymer,
}

#[derive(Serialize, Debug, PartialEq)]
pub enum Material {
    Metal,
    Glass,
    Plastic,
    Aluminum,
}

#[derive(Serialize, Debug, PartialEq)]
pub enum SimCard {
    FullSize,
    Mini,
    Micro,
    Nano,
    Embedded,
}
#[derive(Serialize, Debug, PartialEq)]
pub enum DisplayType {
    Oled,
    Amoled,
    LCD,
    TFT,
    IPS,
    Retina,
}

#[derive(Serialize, Debug, PartialEq)]
pub enum InternetConnectionTechnology {
    GPRS,
    EDGE,
    _3G,
    _4G,
    _5G,
}

#[derive(Serialize, Debug, PartialEq)]
pub enum SatelliteNavigation {
    GPS,
    A_GPS,
    Galileo,
    BeiDou,
    GLONASS,
}

#[derive(Serialize, Debug, PartialEq)]
pub enum WifiStandard {
    _4,
    _5,
    _6,
    _7,
    A,
    B,
    G,
}

#[derive(Serialize, Debug, PartialEq)]
pub enum ChargingConnectorType {
    USBTypeC,
    MicroUSB,
}
#[derive(Serialize, Debug, PartialEq)]
pub enum AudioJack {
    _3_5mm,
}

#[derive(Serialize, Debug, PartialEq)]
pub enum Technology {
    NFC,
    FastCharging,
    InfraredPort,
    WirelessCharger,
    Autofocus,
}

#[derive(Serialize, Debug, PartialEq)]
pub enum Country {
    China,
}

#[derive(Serialize, Debug, PartialEq)]
pub enum MemoryCardSlot {
    Hybrid,
    Separate,
}
#[derive(Serialize, Debug, PartialEq)]
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
}

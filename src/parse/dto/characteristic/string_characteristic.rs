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
}
#[derive(Serialize, Debug, PartialEq)]
pub enum AudioJack {
    _3_5mm,
}

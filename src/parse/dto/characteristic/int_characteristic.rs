use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub enum IntCharacteristic {
    BatteryCapacity_mA_h(i32),
    NumberOfProcessorCores(i32),
    BuiltInMemory_GB(i32),
    Ram_GB(i32),
    FrontCamera_MP(i32),
    VideoResolution_Pix(i32),
    AmountOfSimCards(i32),
    PPI(i32),
    Fps(i32),
    Brightness_cd_m2(i32),
    UpdateFrequency_Hz(i32),
    Camera_mp(i32),
    LTEBand(i32),
    GSMBand(i32),
    Warranty_month(i32),
}

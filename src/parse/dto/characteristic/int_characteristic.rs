use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub enum IntCharacteristic {
    Width(i32),
    Height(i32),
    Thickness(i32),
    BatteryCapacity_mA_h(i32),
    NumberOfProcessorCores(i32),
    BuiltInMemory_GB(i32),
    Ram_GB(i32),
    FrontCamera_MP(i32),
    VideoResolution_Pix(i32),
    Weight_gr(i32),
    MIUIVersion(i32),
    AndroidVersion(i32),
}

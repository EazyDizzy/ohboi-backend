use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub enum FloatCharacteristic {
    Width_mm(f32),
    Height_mm(f32),
    Thickness_mm(f32),
    ScreenDiagonal(f32),
    Bluetooth(f32),
    CPUFrequency_Ghz(f32),
    Weight_gr(f32),
    MIUIVersion(f32),
    AndroidVersion(f32),
    Aperture(f32),
}

#![allow(non_camel_case_types)]

use std::fmt;

use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Copy, Clone)]
pub enum FloatCharacteristic {
    Width_mm(f32),
    Height_mm(f32),
    Thickness_mm(f32),
    ScreenDiagonal(f32),
    BluetoothVersion(f32),
    CPUFrequency_Ghz(f32),
    Weight_gr(f32),
    MIUIVersion(f32),
    AndroidVersion(f32),
    Aperture(f32),
}

impl fmt::Display for FloatCharacteristic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl FloatCharacteristic {
    pub fn name(&self) -> String {
        let name = self.to_string();

        name[0..name.find('(').unwrap()].to_string()
    }

    pub fn value(&self) -> f32 {
        use FloatCharacteristic::*;

        match self {
            Width_mm(n) | Height_mm(n) | Thickness_mm(n) | ScreenDiagonal(n)
            | BluetoothVersion(n) | CPUFrequency_Ghz(n) | Weight_gr(n) | MIUIVersion(n)
            | AndroidVersion(n) | Aperture(n) => *n,
        }
    }
}

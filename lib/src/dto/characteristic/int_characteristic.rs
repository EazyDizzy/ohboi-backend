#![allow(non_camel_case_types)]

use std::fmt;

use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Copy, Clone)]
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
    LTEDiapason(i32),
    GSMDiapason(i32),
    UMTSDiapason(i32),
    Warranty_month(i32),
    MaxMemoryCardSize_GB(i32),
}

impl fmt::Display for IntCharacteristic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl IntCharacteristic {
    pub fn name(&self) -> String {
        let name = self.to_string();

        name[0..name.find('(').unwrap()].to_string()
    }

    pub fn value(&self) -> i32 {
        use IntCharacteristic::*;

        match self {
            BatteryCapacity_mA_h(n)
            | NumberOfProcessorCores(n)
            | BuiltInMemory_GB(n)
            | Ram_GB(n)
            | FrontCamera_MP(n)
            | VideoResolution_Pix(n)
            | AmountOfSimCards(n)
            | PPI(n)
            | Fps(n)
            | Brightness_cd_m2(n)
            | UpdateFrequency_Hz(n)
            | Camera_mp(n)
            | LTEDiapason(n)
            | GSMDiapason(n)
            | UMTSDiapason(n)
            | Warranty_month(n)
            | MaxMemoryCardSize_GB(n) => *n,
        }
    }
}

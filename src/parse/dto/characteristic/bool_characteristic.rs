use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub enum BoolCharacteristic {
    HasFastCharging(bool),
    HasNFC(bool),
    DisplayIsSensor(bool),
    CanShootVideo(bool),
}

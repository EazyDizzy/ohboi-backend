use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub enum BoolCharacteristic {
    FastCharging,
}

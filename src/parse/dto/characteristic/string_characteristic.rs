use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub enum StringCharacteristic {
    BatteryType(BatteryType),
    ConnectorType,
    Material(Vec<Material>),
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

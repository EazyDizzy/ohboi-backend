pub struct IntCharacteristic {
    pub slug: IntCharacteristicSlug,
    pub value: i32,
}

pub enum IntCharacteristicSlug {
    Width,
    Height,
    Thickness,
    BatteryCapacity,
}
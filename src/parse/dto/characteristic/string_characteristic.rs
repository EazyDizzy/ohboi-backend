pub struct StringCharacteristic {
    pub slug: StringCharacteristicSlug,
    pub value: String,
}

pub enum StringCharacteristicSlug {
    BatteryType,
    ConnectorType,
}
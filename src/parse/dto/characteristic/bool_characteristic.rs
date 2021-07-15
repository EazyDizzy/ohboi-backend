pub struct BoolCharacteristic {
    pub slug: BoolCharacteristicSlug,
    pub value: String,
}

pub enum BoolCharacteristicSlug {
    FastCharging,
}
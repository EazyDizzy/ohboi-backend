use serde::Serialize;

use crate::my_enum::{CharacteristicValueType, CharacteristicVisualisationType, CharacteristicGroupSlug};

#[derive(Serialize, Queryable)]
pub struct Characteristic {
    pub id: i16,
    pub slug: String,
    pub enabled: bool,
    pub visualisation_type: CharacteristicVisualisationType,
    pub value_type: CharacteristicValueType,
    pub sort_key: i16,
    pub group_slug: CharacteristicGroupSlug,
}
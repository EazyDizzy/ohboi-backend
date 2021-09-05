use serde::Serialize;

use crate::my_enum::{CharacteristicValueType, CharacteristicVisualisationType};

#[derive(Serialize, Queryable)]
pub struct Characteristic {
    pub id: i16,
    pub slug: String,
    pub enabled: bool,
    pub visualisation_type: CharacteristicVisualisationType,
    pub value_type: CharacteristicValueType,
}
use serde::Serialize;

use crate::my_enum::{CharacteristicValueType, CharacteristicVisualisationType};
use crate::schema::characteristic;

#[derive(Serialize, Queryable)]
pub struct Characteristic {
    pub id: i16,
    pub slug: String,
    pub enabled: bool,
    pub visualisation_type: CharacteristicVisualisationType,
    pub value_type: CharacteristicValueType,
}

#[derive(Insertable, Debug)]
#[table_name = "characteristic"]
pub struct NewCharacteristic {
    pub id: i16,
    pub slug: String,
    pub enabled: bool,
    pub visualisation_type: CharacteristicVisualisationType,

    pub value_type: CharacteristicValueType,
}

use lib::my_enum::{CharacteristicValueType, CharacteristicVisualisationType, CharacteristicGroupSlug};
use lib::schema::characteristic;

#[derive(Insertable, AsChangeset, Debug)]
#[table_name = "characteristic"]
pub struct NewCharacteristic {
    pub id: i16,
    pub slug: String,
    pub enabled: bool,
    pub visualisation_type: CharacteristicVisualisationType,
    pub value_type: CharacteristicValueType,
    pub sort_key: i16,
    pub group_slug: CharacteristicGroupSlug,
}

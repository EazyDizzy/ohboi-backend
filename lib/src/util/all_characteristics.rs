use strum::IntoEnumIterator;
use strum::VariantNames;

use crate::db::entity::characteristic::Characteristic;
use crate::dto::characteristic::enum_characteristic::EnumCharacteristic;
use crate::dto::characteristic::float_characteristic::FloatCharacteristic;
use crate::dto::characteristic::int_characteristic::IntCharacteristic;
use crate::dto::characteristic::string_characteristic::StringCharacteristic;
use crate::dto::characteristic::TypedCharacteristic;
use crate::util::characteristic_id::get_characteristic_id;
use crate::my_enum::{CharacteristicValueType, CharacteristicVisualisationType};

pub fn get_all_characteristics_dto() -> Vec<Characteristic> {
    let mut chars = get_float_characteristics();
    chars.append(&mut get_int_characteristics());
    chars.append(&mut get_string_characteristics());
    chars.append(&mut get_enum_characteristics());

    chars
}

pub fn get_float_characteristics() -> Vec<Characteristic> {
    let mut characteristics = vec![];
    for item in FloatCharacteristic::iter() {
        let value_type = CharacteristicValueType::Float;
        let visualisation_type = get_float_char_vis_type(item);
        let id = get_characteristic_id(TypedCharacteristic::Float(item));

        characteristics.push(Characteristic {
            id,
            slug: item.name(),
            enabled: true,
            visualisation_type,
            value_type,
        });
    }

    characteristics
}
pub fn get_int_characteristics() -> Vec<Characteristic> {
    let mut characteristics = vec![];
    for item in IntCharacteristic::iter() {
        let value_type = CharacteristicValueType::Int;
        let visualisation_type = get_int_char_vis_type(item);
        let id = get_characteristic_id(TypedCharacteristic::Int(item));

        characteristics.push(Characteristic {
            id,
            slug: item.name(),
            enabled: true,
            visualisation_type,
            value_type,
        });
    }

    characteristics
}
pub fn get_string_characteristics() -> Vec<Characteristic> {
    let mut characteristics = vec![];
    for item in StringCharacteristic::iter() {
        let value_type = CharacteristicValueType::String;
        let visualisation_type = CharacteristicVisualisationType::MultiSelector;
        let id = get_characteristic_id(TypedCharacteristic::String(item.clone()));

        characteristics.push(Characteristic {
            id,
            slug: item.name(),
            enabled: true,
            visualisation_type,
            value_type,
        });
    }

    characteristics
}

pub fn get_enum_characteristics() -> Vec<Characteristic> {
    let mut characteristics = vec![];
    for item in EnumCharacteristic::VARIANTS {
        let value_type = CharacteristicValueType::Enum;
        let visualisation_type = CharacteristicVisualisationType::MultiSelector;
        let id = get_characteristic_id(TypedCharacteristic::Enum(
            EnumCharacteristic::type_from_name(item),
        ));

        characteristics.push(Characteristic {
            id,
            slug: item.to_string(),
            enabled: true,
            visualisation_type,
            value_type,
        });
    }

    characteristics
}

fn get_float_char_vis_type(char: FloatCharacteristic) -> CharacteristicVisualisationType {
    use FloatCharacteristic::*;

    match char {
        Width_mm(_) => CharacteristicVisualisationType::Range,
        Height_mm(_) => CharacteristicVisualisationType::Range,
        Thickness_mm(_) => CharacteristicVisualisationType::Range,
        ScreenDiagonal(_) => CharacteristicVisualisationType::MultiSelector,
        BluetoothVersion(_) => CharacteristicVisualisationType::MultiSelector,
        CPUFrequency_Ghz(_) => CharacteristicVisualisationType::MultiSelector,
        Weight_gr(_) => CharacteristicVisualisationType::Range,
        MIUIVersion(_) => CharacteristicVisualisationType::MultiSelector,
        AndroidVersion(_) => CharacteristicVisualisationType::MultiSelector,
        Aperture(_) => CharacteristicVisualisationType::MultiSelector,
    }
}

fn get_int_char_vis_type(char: IntCharacteristic) -> CharacteristicVisualisationType {
    use IntCharacteristic::*;

    match char {
        BatteryCapacity_mA_h(_) => CharacteristicVisualisationType::MultiSelector,
        NumberOfProcessorCores(_) => CharacteristicVisualisationType::MultiSelector,
        BuiltInMemory_GB(_) => CharacteristicVisualisationType::MultiSelector,
        Ram_GB(_) => CharacteristicVisualisationType::MultiSelector,
        FrontCamera_MP(_) => CharacteristicVisualisationType::MultiSelector,
        VideoResolution_Pix(_) => CharacteristicVisualisationType::MultiSelector,
        AmountOfSimCards(_) => CharacteristicVisualisationType::MultiSelector,
        PPI(_) => CharacteristicVisualisationType::MultiSelector,
        Fps(_) => CharacteristicVisualisationType::MultiSelector,
        Brightness_cd_m2(_) => CharacteristicVisualisationType::MultiSelector,
        UpdateFrequency_Hz(_) => CharacteristicVisualisationType::MultiSelector,
        Camera_mp(_) => CharacteristicVisualisationType::MultiSelector,
        LTEDiapason(_) => CharacteristicVisualisationType::MultiSelector,
        GSMDiapason(_) => CharacteristicVisualisationType::MultiSelector,
        UMTSDiapason(_) => CharacteristicVisualisationType::MultiSelector,
        Warranty_month(_) => CharacteristicVisualisationType::MultiSelector,
        MaxMemoryCardSize_GB(_) => CharacteristicVisualisationType::MultiSelector,
    }
}

use strum::IntoEnumIterator;
use strum::VariantNames;

use crate::db::entity::characteristic::Characteristic;
use crate::dto::characteristic::enum_characteristic::EnumCharacteristic;
use crate::dto::characteristic::float_characteristic::FloatCharacteristic;
use crate::dto::characteristic::int_characteristic::IntCharacteristic;
use crate::dto::characteristic::string_characteristic::StringCharacteristic;
use crate::dto::characteristic::{TypedCharacteristic};
use crate::my_enum::{CharacteristicValueType, CharacteristicVisualisationType, CharacteristicGroupSlug};
use crate::util::characteristic_id::get_characteristic_id;

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
        let sort_key = get_float_char_sort_key(&item);
        let group_slug = get_float_char_group_slug(&item);
        let id = get_characteristic_id(&TypedCharacteristic::Float(item));

        characteristics.push(Characteristic {
            id,
            slug: item.name(),
            enabled: true,
            visualisation_type,
            value_type,
            sort_key,
            group_slug,
        });
    }

    characteristics
}

pub fn get_int_characteristics() -> Vec<Characteristic> {
    let mut characteristics = vec![];
    for item in IntCharacteristic::iter() {
        let value_type = CharacteristicValueType::Int;
        let visualisation_type = get_int_char_vis_type(item);
        let sort_key = get_int_char_sort_key(&item);
        let group_slug = get_int_char_group_slug(&item);
        let id = get_characteristic_id(&TypedCharacteristic::Int(item));

        characteristics.push(Characteristic {
            id,
            slug: item.name(),
            enabled: true,
            visualisation_type,
            value_type,
            sort_key,
            group_slug,
        });
    }

    characteristics
}

pub fn get_string_characteristics() -> Vec<Characteristic> {
    let mut characteristics = vec![];
    for item in StringCharacteristic::iter() {
        let value_type = CharacteristicValueType::String;
        let visualisation_type = CharacteristicVisualisationType::MultiSelector;
        let sort_key = get_string_char_sort_key(&item);
        let group_slug = get_string_char_group_slug(&item);
        let id = get_characteristic_id(&TypedCharacteristic::String(item.clone()));

        characteristics.push(Characteristic {
            id,
            slug: item.name(),
            enabled: true,
            visualisation_type,
            value_type,
            sort_key,
            group_slug,
        });
    }

    characteristics
}

pub fn get_enum_characteristics() -> Vec<Characteristic> {
    let mut characteristics = vec![];
    for item in EnumCharacteristic::VARIANTS {
        let enum_type = EnumCharacteristic::type_from_name(item);
        let value_type = CharacteristicValueType::Enum;
        let visualisation_type = CharacteristicVisualisationType::MultiSelector;
        let sort_key = get_enum_char_sort_key(&enum_type);
        let group_slug = get_enum_char_group_slug(&enum_type);
        let id = get_characteristic_id(&TypedCharacteristic::Enum(
            enum_type
        ));

        characteristics.push(Characteristic {
            id,
            slug: item.to_string(),
            enabled: true,
            visualisation_type,
            value_type,
            sort_key,
            group_slug,
        });
    }

    characteristics
}

fn get_float_char_sort_key(char: &FloatCharacteristic) -> i16 {
    use FloatCharacteristic::*;

    match char {
        Width_mm(_) => 10,
        Height_mm(_) => 10,
        Thickness_mm(_) => 10,
        ScreenDiagonal(_) => 0,
        BluetoothVersion(_) => 10,
        CPUFrequency_Ghz(_) => 1,
        Weight_gr(_) => 9,
        MIUIVersion(_) => 2,
        AndroidVersion(_) => 2,
        Aperture(_) => 2,
    }
}
fn get_float_char_group_slug(char: &FloatCharacteristic) -> CharacteristicGroupSlug {
    use FloatCharacteristic::*;

    match char {
        Width_mm(_) => CharacteristicGroupSlug::Appearance,
        Height_mm(_) => CharacteristicGroupSlug::Appearance,
        Thickness_mm(_) => CharacteristicGroupSlug::Appearance,
        ScreenDiagonal(_) => CharacteristicGroupSlug::Display,
        BluetoothVersion(_) => CharacteristicGroupSlug::Sensors,
        CPUFrequency_Ghz(_) => CharacteristicGroupSlug::Processor,
        Weight_gr(_) => CharacteristicGroupSlug::Appearance,
        MIUIVersion(_) => CharacteristicGroupSlug::General,
        AndroidVersion(_) => CharacteristicGroupSlug::General,
        Aperture(_) => CharacteristicGroupSlug::Camera,
    }
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

fn get_int_char_sort_key(char: &IntCharacteristic) -> i16 {
    use IntCharacteristic::*;

    match char {
        BatteryCapacity_mA_h(_) => 0,
        NumberOfProcessorCores(_) => 0,
        BuiltInMemory_GB(_) => 1,
        Ram_GB(_) => 0,
        FrontCamera_MP(_) => 0,
        VideoResolution_Pix(_) => 0,
        AmountOfSimCards(_) => 2,
        PPI(_) => 3,
        Fps(_) => 1,
        Brightness_cd_m2(_) => 3,
        UpdateFrequency_Hz(_) => 1,
        Camera_mp(_) => 0,
        LTEDiapason(_) => 10,
        GSMDiapason(_) => 10,
        UMTSDiapason(_) => 10,
        Warranty_month(_) => 3,
        MaxMemoryCardSize_GB(_) => 1,
    }
}
fn get_int_char_group_slug(char: &IntCharacteristic) -> CharacteristicGroupSlug {
    use IntCharacteristic::*;

    match char {
        BatteryCapacity_mA_h(_) => CharacteristicGroupSlug::Power,
        NumberOfProcessorCores(_) => CharacteristicGroupSlug::Processor,
        BuiltInMemory_GB(_) => CharacteristicGroupSlug::Memory,
        Ram_GB(_) => CharacteristicGroupSlug::Memory,
        FrontCamera_MP(_) => CharacteristicGroupSlug::Camera,
        VideoResolution_Pix(_) => CharacteristicGroupSlug::Camera,
        AmountOfSimCards(_) => CharacteristicGroupSlug::Connection,
        PPI(_) => CharacteristicGroupSlug::Display,
        Fps(_) => CharacteristicGroupSlug::Camera,
        Brightness_cd_m2(_) => CharacteristicGroupSlug::Display,
        UpdateFrequency_Hz(_) => CharacteristicGroupSlug::Display,
        Camera_mp(_) => CharacteristicGroupSlug::Camera,
        LTEDiapason(_) => CharacteristicGroupSlug::Connection,
        GSMDiapason(_) => CharacteristicGroupSlug::Connection,
        UMTSDiapason(_) => CharacteristicGroupSlug::Connection,
        Warranty_month(_) => CharacteristicGroupSlug::General,
        MaxMemoryCardSize_GB(_) => CharacteristicGroupSlug::Memory,
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

fn get_string_char_sort_key(char: &StringCharacteristic) -> i16 {
    use StringCharacteristic::*;

    match char {
        Processor(_) => 0,
        VideoProcessor(_) => 1,
        AspectRatio(_) => 2,
        DisplayResolution(_) => 1,
        Contrast(_) => 5,
        Model(_) => 1,
    }
}
fn get_string_char_group_slug(char: &StringCharacteristic) -> CharacteristicGroupSlug {
    use StringCharacteristic::*;

    match char {
        Processor(_) => CharacteristicGroupSlug::Processor,
        VideoProcessor(_) => CharacteristicGroupSlug::Processor,
        AspectRatio(_) => CharacteristicGroupSlug::Display,
        DisplayResolution(_) => CharacteristicGroupSlug::Display,
        Contrast(_) => CharacteristicGroupSlug::Display,
        Model(_) => CharacteristicGroupSlug::General,
    }
}

fn get_enum_char_sort_key(char: &EnumCharacteristic) -> i16 {
    use EnumCharacteristic::*;

    match char {
        ChargingConnectorType(_) => 1,
        BatteryType(_) => 1,
        SimCard(_) => 2,
        Material(_) => 3,
        DisplayType(_) => 1,
        InternetConnectionTechnology(_) => 1,
        SatelliteNavigation(_) => 10,
        WifiStandard(_) => 3,
        AudioJack(_) => 2,
        TechnologySupport(_) => 1,
        ProducingCountry(_) => 0,
        MemoryCardSlot(_) => 1,
        SupportedMediaFormat(_) => 3,
    }
}
fn get_enum_char_group_slug(char: &EnumCharacteristic) -> CharacteristicGroupSlug {
    use EnumCharacteristic::*;

    match char {
        ChargingConnectorType(_) => CharacteristicGroupSlug::Power,
        BatteryType(_) => CharacteristicGroupSlug::Power,
        SimCard(_) => CharacteristicGroupSlug::Connection,
        Material(_) => CharacteristicGroupSlug::Appearance,
        DisplayType(_) => CharacteristicGroupSlug::Display,
        InternetConnectionTechnology(_) => CharacteristicGroupSlug::Connection,
        SatelliteNavigation(_) => CharacteristicGroupSlug::Connection,
        WifiStandard(_) => CharacteristicGroupSlug::Connection,
        AudioJack(_) => CharacteristicGroupSlug::General,
        TechnologySupport(_) => CharacteristicGroupSlug::General,
        ProducingCountry(_) => CharacteristicGroupSlug::General,
        MemoryCardSlot(_) => CharacteristicGroupSlug::Memory,
        SupportedMediaFormat(_) => CharacteristicGroupSlug::General,
    }
}

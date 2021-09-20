use crate::dto::characteristic::enum_characteristic::EnumCharacteristic;
use crate::dto::characteristic::float_characteristic::FloatCharacteristic;
use crate::dto::characteristic::int_characteristic::IntCharacteristic;
use crate::dto::characteristic::string_characteristic::StringCharacteristic;
use crate::dto::characteristic::TypedCharacteristic;

pub fn get_characteristic_id(char: &TypedCharacteristic) -> i16 {
    match char {
        TypedCharacteristic::Float(v) => match v {
            FloatCharacteristic::Width_mm(_) => 1,
            FloatCharacteristic::Height_mm(_) => 2,
            FloatCharacteristic::Thickness_mm(_) => 3,
            FloatCharacteristic::ScreenDiagonal(_) => 4,
            FloatCharacteristic::BluetoothVersion(_) => 5,
            FloatCharacteristic::CPUFrequency_Ghz(_) => 6,
            FloatCharacteristic::Weight_gr(_) => 7,
            FloatCharacteristic::MIUIVersion(_) => 8,
            FloatCharacteristic::AndroidVersion(_) => 9,
            FloatCharacteristic::Aperture(_) => 10,
        },
        TypedCharacteristic::Int(v) => match v {
            IntCharacteristic::BatteryCapacity_mA_h(_) => 11,
            IntCharacteristic::NumberOfProcessorCores(_) => 12,
            IntCharacteristic::BuiltInMemory_GB(_) => 13,
            IntCharacteristic::Ram_GB(_) => 14,
            IntCharacteristic::FrontCamera_MP(_) => 15,
            IntCharacteristic::VideoResolution_Pix(_) => 16,
            IntCharacteristic::AmountOfSimCards(_) => 17,
            IntCharacteristic::PPI(_) => 18,
            IntCharacteristic::Fps(_) => 19,
            IntCharacteristic::Brightness_cd_m2(_) => 20,
            IntCharacteristic::UpdateFrequency_Hz(_) => 21,
            IntCharacteristic::Camera_mp(_) => 22,
            IntCharacteristic::LTEDiapason(_) => 23,
            IntCharacteristic::GSMDiapason(_) => 24,
            IntCharacteristic::UMTSDiapason(_) => 25,
            IntCharacteristic::Warranty_month(_) => 26,
            IntCharacteristic::MaxMemoryCardSize_GB(_) => 27,
        },
        TypedCharacteristic::String(v) => match v {
            StringCharacteristic::Processor(_) => 28,
            StringCharacteristic::VideoProcessor(_) => 29,
            StringCharacteristic::AspectRatio(_) => 30,
            StringCharacteristic::DisplayResolution(_) => 31,
            StringCharacteristic::Contrast(_) => 32,
            StringCharacteristic::Model(_) => 33,
        },
        TypedCharacteristic::Enum(v) => match v {
            EnumCharacteristic::ChargingConnectorType(_) => 34,
            EnumCharacteristic::BatteryType(_) => 35,
            EnumCharacteristic::SimCard(_) => 36,
            EnumCharacteristic::Material(_) => 37,
            EnumCharacteristic::DisplayType(_) => 38,
            EnumCharacteristic::InternetConnectionTechnology(_) => 39,
            EnumCharacteristic::SatelliteNavigation(_) => 40,
            EnumCharacteristic::WifiStandard(_) => 41,
            EnumCharacteristic::AudioJack(_) => 42,
            EnumCharacteristic::TechnologySupport(_) => 43,
            EnumCharacteristic::ProducingCountry(_) => 44,
            EnumCharacteristic::MemoryCardSlot(_) => 45,
            EnumCharacteristic::SupportedMediaFormat(_) => 46,
        },
    }
}

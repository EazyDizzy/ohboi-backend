use std::fmt;

use serde::Serialize;
use strum_macros::EnumIter;

#[derive(Serialize, Debug, PartialEq, EnumIter, Clone)]
pub enum StringCharacteristic {
    Processor(String),
    VideoProcessor(String),
    AspectRatio(String),
    DisplayResolution(String),
    Contrast(String),
    Model(String),
}

impl fmt::Display for StringCharacteristic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl StringCharacteristic {
    pub fn name(&self) -> String {
        let name = self.to_string();

        name[0..name.find("(").unwrap()].to_string()
    }

    pub fn value(&self) -> String {
        use StringCharacteristic::*;

        match self {
            Processor(n) | VideoProcessor(n) | AspectRatio(n) | DisplayResolution(n)
            | Contrast(n) | Model(n) => n.clone(),
        }
    }
}

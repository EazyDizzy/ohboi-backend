use inflector::Inflector;

use lib::error_reporting::DisplayString;

pub mod cloud;
pub mod html_cleaner;
pub mod request;

#[derive(Debug)]
enum Executor {
    Cloud,
}

impl DisplayString for Executor {
    fn to_display_string(&self) -> String {
        format!("service::{}", format!("{:?}", self).to_snake_case())
    }
}

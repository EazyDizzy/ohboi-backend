use inflector::Inflector;

use lib::error_reporting::DisplayString;
use std::fmt::Debug;

pub mod entity;
pub mod repository;

#[derive(Debug)]
enum Executor {
    Characteristic,
}

impl DisplayString for Executor {
    fn to_display_string(&self) -> String {
        format!("db::{}", format!("{:?}", self).to_snake_case())
    }
}

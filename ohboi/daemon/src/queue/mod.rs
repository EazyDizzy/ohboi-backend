use lib::error_reporting::DisplayString;
pub use pub_api::*;

use crate::{ConsumerName, ProducerName};
use inflector::Inflector;

mod consumer;
mod layer;
mod producer;
mod pub_api;

impl DisplayString for ConsumerName {
    fn to_display_string(&self) -> String {
       format!("consumer::{}", self.to_string().to_snake_case())
    }
}

impl DisplayString for ProducerName {
    fn to_display_string(&self) -> String {
       format!("consumer::{}", self.to_string().to_snake_case())
    }
}


#[derive(Debug)]
enum Executor {
    Queue,
    PullExchangeRates
}

impl DisplayString for Executor {
    fn to_display_string(&self) -> String {
        format!("queue::{}", format!("{:?}", self).to_snake_case())
    }
}

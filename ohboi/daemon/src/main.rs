#![deny(clippy::all, clippy::pedantic, clippy::cognitive_complexity)]
#![allow(
    clippy::module_name_repetitions,
    clippy::default_trait_access,
    clippy::module_inception,
    clippy::too_many_lines,
    clippy::await_holding_lock,
    clippy::expect_fun_call,
    clippy::semicolon_if_nothing_returned,
    clippy::non_ascii_literal
)]
#![warn(unused_extern_crates)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

use clap::arg_enum;
use structopt::StructOpt;

use crate::db::repository::sync_characteristic_enum;
use crate::queue::declare::declare_all_queues;
use crate::queue::launch::{launch_consumer, launch_producer};
use crate::settings::Settings;

mod db;
mod dto;
mod parse;
mod queue;
mod service;
mod settings;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(possible_values = & ["consumer", "producer", "queue_config", "characteristic_enum_sync"], case_insensitive = true)]
    worker_type: String,
    #[structopt(short, possible_values = & ConsumerName::variants(), case_insensitive = true, required_if("worker-type", "consumer"))]
    consumer_name: Option<ConsumerName>,
    #[structopt(short, possible_values = & ProducerName::variants(), case_insensitive = true, required_if("worker-type", "producer"))]
    producer_name: Option<ProducerName>,
}
arg_enum! {
    #[derive(Debug, Copy, Clone)]
    pub enum ConsumerName {
        ParseCategory,
        ParseImage,
        ParsePage,
        ParseDetails,
        PullExchangeRates,
    }
}

arg_enum! {
    #[derive(Debug, Copy, Clone)]
    pub enum ProducerName {
        ParseCategory,
        PullExchangeRates,
    }
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "daemon");
    env_logger::init();
    let guard = lib::error_reporting::init();

    let args: Cli = Cli::from_args();

    if args.worker_type == "characteristic_enum_sync" {
        sync_characteristic_enum();
        return;
    }
    if args.worker_type == "queue_config" {
        declare_all_queues().await;
        return;
    }

    if args.worker_type == "producer" {
        let name = args.producer_name.expect("Failed to daemon producer name.");

        launch_producer(name)
            .await
            .expect(&format!("[{}] Failed to run producer.", &name));
    } else {
        let name = args.consumer_name.expect("Failed to daemon consumer name.");

        launch_consumer(name)
            .await
            .expect(&format!("[{}] Failed to run consumer.", &name));
    }

    guard.close(None);
}

lazy_static! {
    static ref SETTINGS: Settings = Settings::new();
}

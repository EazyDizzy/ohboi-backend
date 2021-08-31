#![deny(clippy::all, clippy::pedantic, clippy::cognitive_complexity)]
#![allow(
    clippy::module_name_repetitions,
    clippy::default_trait_access,
    clippy::module_inception,
    clippy::too_many_lines,
    clippy::await_holding_lock
)]
#![warn(unused_extern_crates)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

use clap::arg_enum;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use structopt::StructOpt;

use parse::settings::Settings;

use crate::parse::db::repository::sync_characteristic_enum;
use crate::parse::queue::pub_api::declare::{declare_queue, declare_all_queues};
use crate::parse::queue::pub_api::launch::{start_consumer, start_producer};

mod common;
mod local_sentry;
mod my_enum;
mod parse;
mod schema;

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
    let guard = local_sentry::init_sentry();

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
        let name = args.producer_name.expect("Failed to parse producer name.");

        let _res = start_producer(name)
            .await
            .expect(&format!("[{}] Failed to run producer.", &name));
    } else {
        let name = args.consumer_name.expect("Failed to parse consumer name.");

        let _res = start_consumer(name)
            .await
            .expect(&format!("[{}] Failed to run consumer.", &name));
    }

    let close_result = guard.close(None);
    println!("sentry closed {}", close_result);
}

lazy_static! {
    static ref SETTINGS: Settings = Settings::new().unwrap();
}

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
lazy_static! {
    static ref POOL: Pool = {
        let database_url = &SETTINGS.database.url;
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool")
    };
}

#![deny(clippy::all, clippy::pedantic, clippy::cognitive_complexity)]
#![allow(
    clippy::module_name_repetitions,
    clippy::too_many_lines
)]
#![warn(unused_extern_crates)]
#[macro_use]
extern crate diesel;

use log::{error, info};

use lib::error_reporting;
use lib::error_reporting::DisplayString;

mod auth;
mod db;
mod dto;
mod endpoint;
mod util;

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "http,actix_web=debug");
    env_logger::init();
    let _guard = error_reporting::init();

    let result = endpoint::run_server().await;
    match result {
        Ok(_) => info!("Server started."),
        Err(e) => error!("Server failed: {:?}", e),
    }
}

#[derive(Debug)]
enum Executor {
    GoogleAuth,
}

impl DisplayString for Executor {
    fn to_display_string(&self) -> String {
        format!("http::{:?}", self)
    }
}

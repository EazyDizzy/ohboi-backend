#![deny(clippy::all, clippy::pedantic, clippy::cognitive_complexity)]
#![allow(
    clippy::module_name_repetitions,
    clippy::default_trait_access,
    clippy::module_inception,
    clippy::too_many_lines
)]
#![warn(unused_extern_crates)]
#[macro_use]
extern crate diesel;

use std::env;

use log::{error, info};

use lib::local_sentry;

mod auth;
mod db;
mod dto;
mod endpoint;
mod util;

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "http,actix_web=debug");
    env_logger::init();
    let _guard = local_sentry::init_sentry();

    let result = endpoint::run_server().await;
    match result {
        Ok(_) => info!("Server started."),
        Err(e) => error!("Server failed: {:?}", e),
    }
}

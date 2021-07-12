#![deny(clippy::all, clippy::pedantic, clippy::cognitive_complexity)]
#![allow(clippy::module_name_repetitions, clippy::default_trait_access, clippy::module_inception, clippy::too_many_lines)]
#![warn(unused_extern_crates)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

use std::env;

use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use log::{error, info};

mod http;
mod schema;
mod my_enum;
mod local_sentry;
mod common;

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "http,actix_web=debug");
    env_logger::init();
    let _guard = local_sentry::init_sentry();

    let result = http::run_server().await;
    match result {
        Ok(_) => info!("Server started."),
        Err(e) => error!("Server failed: {:?}", e)
    }
}

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
lazy_static! {
    static ref POOL: Pool = {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        r2d2::Pool::builder()
                .build(manager)
                .expect("Failed to create pool")
    };
}
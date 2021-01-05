#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;

use std::env;
use std::sync::Arc;
use std::time::Instant;

use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use r2d2;

use crate::parse::crawler::mi_shop_com::MiShopComCrawler;

mod http;
mod db;
mod schema;
mod parse;

#[actix_web::main]
async fn main() {
    dotenv().ok();

    let _guard = sentry::init(
        sentry::ClientOptions {
            attach_stacktrace: true,
            send_default_pii: true,
            auto_session_tracking: true,
            release: Some(env::var("CARGO_PKG_VERSION").unwrap().into()),

            before_send: Some(Arc::new(|event| {
                if event.message.is_some() {
                    println!("sentry: {:?}", event.message.clone().unwrap());
                }

                Some(event)
            })),
            ..Default::default()
        }
    );

    let parse_start = Instant::now();
    let parse_result = parse::parser::parse(&MiShopComCrawler {}).await;
    println!("Parse time: {}s", parse_start.elapsed().as_secs());

    match parse_result {
        Ok(_) => println!("Parsed"),
        Err(e) => println!("Parsing failed: {}", e)
    }

    let result = http::run_server().await;
    match result {
        Ok(_) => println!("Server started."),
        Err(e) => println!("Server failed: {}", e)
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
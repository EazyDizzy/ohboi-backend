#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate log;
extern crate maplit;

use std::env;
use std::sync::Arc;
use std::time::Instant;

use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use log::{debug, error, info};
use r2d2;
use termion::{color, style};

use crate::parse::crawler::mi_shop_com::MiShopComCrawler;

mod http;
mod db;
mod schema;
mod parse;
mod my_enum;

#[actix_web::main]
async fn main() {
    let _guard = sentry::init(
        sentry::ClientOptions {
            attach_stacktrace: true,
            send_default_pii: true,
            auto_session_tracking: true,
            release: Some(env::var("CARGO_PKG_VERSION").unwrap().into()),

            before_send: Some(Arc::new(|event| {
                if event.message.is_some() {
                    error!(
                        "sentry: {}{:#?}{}",
                        color::Fg(color::Red),
                        event.message.clone().unwrap(),
                        style::Reset
                    );
                }

                Some(event)
            })),
            before_breadcrumb: Some(Arc::new(|breadcrumb| {
                if breadcrumb.message.is_some() {
                    info!(
                        "sentry: {}{}{} {}{}{} {}{:?}{}",
                        color::Fg(color::Magenta),
                        breadcrumb.category.clone().unwrap(),
                        style::Reset,
                        //
                        color::Fg(color::Yellow),
                        breadcrumb.message.clone().unwrap(),
                        style::Reset,
                        //
                        color::Fg(color::LightBlue),
                        breadcrumb.data,
                        style::Reset,
                    );
                }

                Some(breadcrumb)
            })),
            ..Default::default()
        }
    );
    env_logger::init();
    let res = parse::producer::crawler_category::start().await;
    println!("producer result: {:?}", res);
    let res2 = parse::consumer::crawler_category::start().await;
    println!("consumer result: {:?}", res2);

    let parse_start = Instant::now();
    let parse_result = parse::parser::parse(&MiShopComCrawler {}).await;
    debug!("Parse time: {}s", parse_start.elapsed().as_secs());

    if parse_result.is_err() {
        error!("Parsing failed: {:?}", parse_result.err())
    }

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
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;

use std::env;
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
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        r2d2::Pool::builder()
                .build(manager)
                .expect("Failed to create pool")
    };
}
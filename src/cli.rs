#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate log;
extern crate maplit;

use std::env;

use clap::arg_enum;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2;
use structopt::StructOpt;

mod db;
mod schema;
mod parse;
mod my_enum;
mod local_sentry;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(possible_values = & ["consumer", "producer"], case_insensitive = true)]
    worker_type: String,
    worker_amount: i32,
    #[structopt(short, possible_values = & ConsumerName::variants(), case_insensitive = true, required_if("worker-type", "consumer"))]
    consumer_name: Option<ConsumerName>,
    #[structopt(short, possible_values = & ProducerName::variants(), case_insensitive = true, required_if("worker-type", "producer"))]
    producer_name: Option<ProducerName>,
}

arg_enum! {
    #[derive(Debug)]
    enum ConsumerName {
        CrawlerCategory,
    }
}

arg_enum! {
    #[derive(Debug)]
    enum ProducerName {
        CrawlerCategory,
    }
}
arg_enum! {
    #[derive(Debug)]
    enum WorkerType {
        Consumer,
        Producer,
    }
}

#[actix_web::main]
async fn main() {
    env_logger::init();
    let _guard = local_sentry::init_sentry();

    let args: Cli = Cli::from_args();

    if args.worker_type == "producer" {
        match args.producer_name.unwrap() {
            ProducerName::CrawlerCategory => {
                let _res = parse::producer::crawler_category::start().await;
            }
        }
    } else {
        match args.consumer_name.unwrap() {
            ConsumerName::CrawlerCategory => {
                let _res = parse::consumer::crawler_category::start().await;
            }
        }
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

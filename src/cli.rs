#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate log;
extern crate maplit;

use clap::arg_enum;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2;
use structopt::StructOpt;

use parse::settings::Settings;

use crate::parse::queue::{declare_parse_category_queue, declare_parse_image_queue, declare_parse_page_queue};

mod schema;
mod parse;
mod my_enum;
mod local_sentry;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(possible_values = & ["consumer", "producer", "queue_config"], case_insensitive = true)]
    worker_type: String,
    #[structopt(short, possible_values = & ConsumerName::variants(), case_insensitive = true, required_if("worker-type", "consumer"))]
    consumer_name: Option<ConsumerName>,
    #[structopt(short, possible_values = & ProducerName::variants(), case_insensitive = true, required_if("worker-type", "producer"))]
    producer_name: Option<ProducerName>,
}

arg_enum! {
    #[derive(Debug)]
    enum ConsumerName {
        ParseCategory,
        ParseImage,
        ParsePage,
    }
}

arg_enum! {
    #[derive(Debug)]
    enum ProducerName {
        ParseCategory,
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
    std::env::set_var("RUST_LOG", "daemon");
    env_logger::init();
    let _guard = local_sentry::init_sentry();

    let args: Cli = Cli::from_args();

    if args.worker_type == "queue_config" {
        let declare1 = declare_parse_category_queue().await;
        let declare2 = declare_parse_image_queue().await;
        let declare3 = declare_parse_page_queue().await;

        if declare1.is_err() || declare2.is_err() || declare3.is_err() {
            log::error!("Queue declaration failed.");
            log::error!("parse_category: {:?}", declare1);
            log::error!("parse_image: {:?}", declare2);
            log::error!("parse_page: {:?}", declare3);
        }
        return;
    }

    if args.worker_type == "producer" {
        match args.producer_name.unwrap() {
            ProducerName::ParseCategory => {
                let _res = parse::producer::parse_category::start().await;
            }
        }
    } else {
        match args.consumer_name.unwrap() {
            ConsumerName::ParseCategory => {
                let _res = parse::consumer::parse_category::start().await;
            }
            ConsumerName::ParseImage => {
                let _res = parse::consumer::parse_image::start().await;
            }
            ConsumerName::ParsePage => {
                let _res = parse::consumer::parse_page::start().await;
            }
        }
    }

    let close_result = _guard.close(None);
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
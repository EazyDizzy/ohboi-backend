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

use crate::parse::queue::{declare_crawler_category_queue, declare_image_upload_queue};

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
        CrawlerCategory,
        ImageUpload,
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

    if args.worker_type == "queue_config" {
        let declare1 = declare_crawler_category_queue().await;
        let declare2 = declare_image_upload_queue().await;

        if declare1.is_err() || declare2.is_err() {
            log::error!("Queue declaration failed");
        }
        return;
    }

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
            ConsumerName::ImageUpload => {
                let _res = parse::consumer::image_upload::start().await;
            }
        }
    }
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
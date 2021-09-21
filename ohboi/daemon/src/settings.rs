use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
    pub product_save_concurrency: usize,
}

#[derive(Debug, Deserialize)]
pub struct QueueSettings {
    pub name: String,
    pub prefetch: u16,
    pub concurrency: usize,
}

#[derive(Debug, Deserialize)]
pub struct Queues {
    pub parse_category: QueueSettings,
    pub pull_exchange_rates: QueueSettings,
    pub parse_image: QueueSettings,
    pub parse_page: QueueSettings,
    pub parse_details: QueueSettings,
}

#[derive(Debug, Deserialize)]
pub struct QueueBroker {
    pub url: String,
    pub queues: Queues,
}

#[derive(Debug, Deserialize)]
pub struct S3 {
    pub bucket: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
    pub queue_broker: QueueBroker,
    pub s3: S3,
}

impl Settings {
    pub fn new() -> Self {
        let database_settings = Database {
            url: dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            product_save_concurrency: dotenv::var("DATABASE_PRODUCT_SAVE_CONCURRENCY")
                .or_else::<String, _>(|_| Ok(String::from("15")))
                .unwrap()
                .parse()
                .unwrap(),
        };
        let s3_settings = S3 {
            bucket: dotenv::var("S3_BUCKET").expect("S3_BUCKET must be set"),
        };

       Settings {
            database: database_settings,
            queue_broker: Settings::get_amqp_settings(),
            s3: s3_settings,
        }
    }

    fn get_amqp_settings() -> QueueBroker {
        let parse_category_settings = QueueSettings {
            name: dotenv::var("AMQP_PARSE_CATEGORY_QUEUE_NAME")
                .or_else::<String, _>(|_| Ok(String::from("parse.category")))
                .unwrap(),
            prefetch: dotenv::var("AMQP_PARSE_CATEGORY_QUEUE_PREFETCH_SIZE")
                .or_else::<String, _>(|_| Ok(String::from("2")))
                .unwrap()
                .parse()
                .unwrap(),
            concurrency: dotenv::var("AMQP_PARSE_CATEGORY_CONCURRENCY")
                .or_else::<String, _>(|_| Ok(String::from("1")))
                .unwrap()
                .parse()
                .unwrap(),
        };
        let parse_upload_settings = QueueSettings {
            name: dotenv::var("AMQP_PARSE_IMAGE_QUEUE_NAME")
                .or_else::<String, _>(|_| Ok(String::from("parse.image")))
                .unwrap(),
            prefetch: dotenv::var("AMQP_PARSE_IMAGE_QUEUE_PREFETCH_SIZE")
                .or_else::<String, _>(|_| Ok(String::from("2")))
                .unwrap()
                .parse()
                .unwrap(),
            concurrency: dotenv::var("AMQP_PARSE_IMAGE_CONCURRENCY")
                .or_else::<String, _>(|_| Ok(String::from("10")))
                .unwrap()
                .parse()
                .unwrap(),
        };
        let parse_page_settings = QueueSettings {
            name: dotenv::var("AMQP_PARSE_PAGE_QUEUE_NAME")
                .or_else::<String, _>(|_| Ok(String::from("parse.page")))
                .unwrap(),
            prefetch: dotenv::var("AMQP_PARSE_PAGE_QUEUE_PREFETCH_SIZE")
                .or_else::<String, _>(|_| Ok(String::from("2")))
                .unwrap()
                .parse()
                .unwrap(),
            concurrency: dotenv::var("AMQP_PARSE_PAGE_CONCURRENCY")
                .or_else::<String, _>(|_| Ok(String::from("1")))
                .unwrap()
                .parse()
                .unwrap(),
        };
        let parse_details_settings = QueueSettings {
            name: dotenv::var("AMQP_PARSE_DETAILS_QUEUE_NAME")
                .or_else::<String, _>(|_| Ok(String::from("parse.details")))
                .unwrap(),
            prefetch: dotenv::var("AMQP_PARSE_DETAILS_QUEUE_PREFETCH_SIZE")
                .or_else::<String, _>(|_| Ok(String::from("10")))
                .unwrap()
                .parse()
                .unwrap(),
            concurrency: dotenv::var("AMQP_PARSE_DETAILS_CONCURRENCY")
                .or_else::<String, _>(|_| Ok(String::from("10")))
                .unwrap()
                .parse()
                .unwrap(),
        };

        let pull_exchange_rates_settings = QueueSettings {
            name: dotenv::var("AMQP_PULL_EXCHANGE_RATES_QUEUE_NAME")
                .or_else::<String, _>(|_| Ok(String::from("pull.exchange_rates")))
                .unwrap(),
            prefetch: dotenv::var("AMQP_PULL_EXCHANGE_RATES_QUEUE_PREFETCH_SIZE")
                .or_else::<String, _>(|_| Ok(String::from("1")))
                .unwrap()
                .parse()
                .unwrap(),
            concurrency: dotenv::var("AMQP_PULL_EXCHANGE_RATES_CONCURRENCY")
                .or_else::<String, _>(|_| Ok(String::from("1")))
                .unwrap()
                .parse()
                .unwrap(),
        };

        let queue_settings = Queues {
            parse_category: parse_category_settings,
            pull_exchange_rates: pull_exchange_rates_settings,
            parse_image: parse_upload_settings,
            parse_page: parse_page_settings,
            parse_details: parse_details_settings,
        };
        QueueBroker {
            url: dotenv::var("AMQP_ADDR").expect("AMQP_ADDR must be set"),
            queues: queue_settings,
        }
    }
}

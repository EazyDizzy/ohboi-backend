use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
    pub product_save_concurrency: usize,
}

#[derive(Debug, Deserialize)]
pub struct AmqpQueueSettings {
    pub name: String,
    pub prefetch: u16,
}

#[derive(Debug, Deserialize)]
pub struct AmqpQueues {
    pub parse_category: AmqpQueueSettings,
    pub parse_image: AmqpQueueSettings,
    pub parse_page: AmqpQueueSettings,
}

#[derive(Debug, Deserialize)]
pub struct Amqp {
    pub url: String,
    pub queues: AmqpQueues,
}

#[derive(Debug, Deserialize)]
pub struct S3 {
    pub bucket: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
    pub amqp: Amqp,
    pub s3: S3,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let database_settings = Database {
            url: dotenv::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            product_save_concurrency: dotenv::var("DATABASE_PRODUCT_SAVE_CONCURRENCY")
                .or_else::<String, _>(|_| {
                    Ok(String::from("15"))
                })
                .unwrap().parse().unwrap(),
        };
        let s3_settings = S3 {
            bucket: dotenv::var("S3_BUCKET")
                .expect("S3_BUCKET must be set")
        };

        Ok(Settings {
            database: database_settings,
            amqp: Settings::get_amqp_settings(),
            s3: s3_settings,
        })
    }

    fn get_amqp_settings() -> Amqp {
        let parse_category_settings = AmqpQueueSettings {
            name: dotenv::var("AMQP_PARSE_CATEGORY_QUEUE_NAME")
                .or_else::<String, _>(|_| {
                    Ok(String::from("parse.category"))
                }).unwrap(),
            prefetch: dotenv::var("AMQP_PARSE_CATEGORY_QUEUE_PREFETCH_SIZE")
                .or_else::<String, _>(|_| {
                    Ok(String::from("2"))
                })
                .unwrap().parse().unwrap(),
        };
        let parse_upload_settings = AmqpQueueSettings {
            name: dotenv::var("AMQP_PARSE_IMAGE_QUEUE_NAME")
                .or_else::<String, _>(|_| {
                    Ok(String::from("parse.image"))
                }).unwrap(),
            prefetch: dotenv::var("AMQP_PARSE_IMAGE_QUEUE_PREFETCH_SIZE")
                .or_else::<String, _>(|_| {
                    Ok(String::from("2"))
                })
                .unwrap().parse().unwrap(),
        };
        let parse_page_settings = AmqpQueueSettings {
            name: dotenv::var("AMQP_PARSE_PAGE_QUEUE_NAME")
                .or_else::<String, _>(|_| {
                    Ok(String::from("parse.page"))
                }).unwrap(),
            prefetch: dotenv::var("AMQP_PARSE_PAGE_QUEUE_PREFETCH_SIZE")
                .or_else::<String, _>(|_| {
                    Ok(String::from("2"))
                })
                .unwrap().parse().unwrap(),
        };

        let queue_settings = AmqpQueues {
            parse_category: parse_category_settings,
            parse_image: parse_upload_settings,
            parse_page: parse_page_settings,
        };
        Amqp {
            url: dotenv::var("AMQP_ADDR")
                .expect("AMQP_ADDR must be set"),
            queues: queue_settings,
        }
    }
}
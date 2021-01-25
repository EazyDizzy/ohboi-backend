use std::env;

use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct AmqpQueueSettings {
    pub name: String,
    pub prefetch: u16,
}

#[derive(Debug, Deserialize)]
pub struct AmqpQueues {
    pub crawler_category: AmqpQueueSettings
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
            url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set")
        };
        let s3_settings = S3 {
            bucket: env::var("S3_BUCKET")
                .expect("S3_BUCKET must be set")
        };

        Ok(Settings {
            database: database_settings,
            amqp: Settings::get_amqp_settings(),
            s3: s3_settings,
        })
    }

    fn get_amqp_settings() -> Amqp {
        let crawler_category_settings = AmqpQueueSettings {
            name: env::var("AMQP_CRAWLER_CATEGORY_QUEUE_NAME")
                .or_else::<String, _>(|_| {
                    Ok(String::from("crawler_category"))
                }).unwrap(),
            prefetch: env::var("AMQP_CRAWLER_CATEGORY_QUEUE_PREFETCH_SIZE")
                .or_else::<String, _>(|_| {
                    Ok(String::from("2"))
                })
                .unwrap().parse().unwrap(),
        };

        let queue_settings = AmqpQueues { crawler_category: crawler_category_settings };
        Amqp {
            url: env::var("AMQP_ADDR")
                .expect("AMQP_ADDR must be set"),
            queues: queue_settings,
        }
    }
}
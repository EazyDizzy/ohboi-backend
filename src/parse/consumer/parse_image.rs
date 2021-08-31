use std::str;

use maplit::btreemap;
use sentry::protocol::map::BTreeMap;
use serde::{Deserialize, Serialize};
use tokio::runtime::Handle;

use crate::local_sentry::add_category_breadcrumb;
use crate::parse::db::entity::source::SourceName;
use crate::parse::db::repository::product::add_image_to_product_details;
use crate::parse::db::repository::source_product::get_by_source_and_external_id;
use crate::parse::service::cloud_uploader::upload_image_to_cloud;
use crate::SETTINGS;
use crossbeam::channel;
use crate::parse::consumer::layer::consume::consume;

#[derive(Serialize, Deserialize, Clone)]
pub struct UploadImageMessage {
    pub file_path: String,
    pub image_url: String,
    pub external_id: String,
    pub source: SourceName,
}

pub async fn start() -> core::result::Result<(), ()> {
    let _ = consume(&SETTINGS.amqp.queues.parse_image, |message| {
        let (snd, rcv) = channel::bounded(1);

        let _ = Handle::current().spawn(async move {
            let message: UploadImageMessage = serde_json::from_str(&message).unwrap();

            let rs = execute(message).await;
            let _ = snd.send(rs);
        });

        rcv.recv().unwrap()
    })
    .await;

    Ok(())
}

async fn execute(message: UploadImageMessage) -> Result<(), ()> {
    let result = upload_image_to_cloud(message.file_path.clone(), message.image_url).await;

    if result {
        let source_product =
            get_by_source_and_external_id(message.source, message.external_id).unwrap();
        add_consumer_breadcrumb(
            "updating product",
            btreemap! {
                "id" => source_product.product_id.to_string(),
            },
        );
        add_image_to_product_details(source_product.product_id, &message.file_path);

        Ok(())
    } else {
        Err(())
    }
}

fn add_consumer_breadcrumb(message: &str, data: BTreeMap<&str, String>) {
    add_category_breadcrumb(
        message,
        data,
        ["consumer.", &SETTINGS.amqp.queues.parse_image.name].join(""),
    );
}

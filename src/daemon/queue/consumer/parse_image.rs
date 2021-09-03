use std::str;

use maplit::btreemap;
use sentry::protocol::map::BTreeMap;
use serde::{Deserialize, Serialize};

use crate::daemon::db::entity::source::SourceName;
use crate::daemon::db::repository::product::add_image_to_product_details;
use crate::daemon::db::repository::source_product::get_by_source_and_external_id;
use crate::daemon::queue::layer::consume::consume;
use crate::daemon::service::cloud::pub_api::upload_image_to_cloud;
use crate::local_sentry::add_category_breadcrumb;
use crate::SETTINGS;

#[derive(Serialize, Deserialize, Clone)]
pub struct UploadImageMessage {
    pub file_path: String,
    pub image_url: String,
    pub external_id: String,
    pub source: SourceName,
}

pub async fn start() -> core::result::Result<(), ()> {
    let _ = consume(&SETTINGS.queue_broker.queues.parse_image, execute)
        .await
        .expect("Can't launch consumer");

    Ok(())
}

async fn execute(message: String) -> Result<(), ()> {
    let message: UploadImageMessage =
        serde_json::from_str(&message).expect("Failed to daemon UploadImageMessage");
    let result = upload_image_to_cloud(message.file_path.clone(), message.image_url).await;

    if result {
        let source_product = get_by_source_and_external_id(message.source, &message.external_id)
            .expect(&format!(
                "SourceProduct doesn't exist. source: {} external_id: {}",
                message.source, &message.external_id
            ));
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
        ["consumer.", &SETTINGS.queue_broker.queues.parse_image.name].join(""),
    );
}

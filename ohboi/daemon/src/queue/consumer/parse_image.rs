use std::collections::BTreeMap;
use std::str;

use maplit::btreemap;
use serde::{Deserialize, Serialize};

use lib::error_reporting;
use lib::error_reporting::ReportingContext;

use crate::db::entity::source::SourceName;
use crate::db::repository::product::add_image_to_product_details;
use crate::db::repository::source_product::get_by_source_and_external_id;
use crate::queue::layer::consume::consume;
use crate::service::cloud::upload_image_to_cloud;
use crate::{ConsumerName, SETTINGS};

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

async fn execute(message: UploadImageMessage) -> Result<(), ()> {
    add_consumer_breadcrumb(
        "downloading image",
        btreemap! {
                "url" => message.image_url.clone(),
            },
        "upload_image"
    );
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
            "update_product"
        );
        add_image_to_product_details(source_product.product_id, &message.file_path);

        Ok(())
    } else {
        Err(())
    }
}

fn add_consumer_breadcrumb(message: &str, data: BTreeMap<&str, String>, action: &str) {
    error_reporting::add_breadcrumb(
        message,
        data,
        &ReportingContext {
            executor: &ConsumerName::ParseImage,
            action,
        },
    );
}

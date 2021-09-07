use lapin::Result;
use maplit::btreemap;

use lib::error_reporting::{add_breadcrumb, ReportingContext};
use crate::db::entity::category::CategorySlug;
use crate::db::entity::source::SourceName;
use crate::queue::consumer::parse_details::ParseDetailsMessage;
use crate::queue::consumer::parse_image::UploadImageMessage;
use crate::queue::consumer::parse_page::ParsePageMessage;
use crate::queue::layer::produce::produce;
use crate::{SETTINGS, ConsumerName};
use std::collections::BTreeMap;

pub async fn postpone_page_parsing(
    url: String,
    source: SourceName,
    category: CategorySlug,
) -> Result<()> {
    let message = ParsePageMessage {
        url,
        source,
        category,
    };
    let breadcrumb_data = btreemap! {
        "category" => message.category.to_string(),
        "source" => message.source.to_string(),
        "url" => message.url.clone()
    };
    add_consumer_breadcrumb(
        "postponing page parsing",
        breadcrumb_data,
        ConsumerName::ParsePage,
    );

    produce(&SETTINGS.queue_broker.queues.parse_page, &message).await
}
pub async fn postpone_details_parsing(
    external_id: String,
    source: SourceName,
    product_id: i32,
) -> Result<()> {
    let message = ParseDetailsMessage {
        external_id,
        source,
        product_id,
    };
    let breadcrumb_data = btreemap! {
        "source" => message.source.to_string(),
        "external_id" => message.external_id.to_string()
    };
    add_consumer_breadcrumb(
        "postponing details parsing",
        breadcrumb_data,
        ConsumerName::ParseDetails,
    );
    produce(&SETTINGS.queue_broker.queues.parse_details, &message).await
}

pub async fn postpone_image_parsing(
    file_path: String,
    image_url: String,
    external_id: String,
    source: SourceName,
) -> Result<()> {
    let message = UploadImageMessage {
        file_path,
        image_url,
        external_id,
        source,
    };
    let breadcrumb_data = btreemap! {
        "file_path" => message.file_path.clone(),
        "image_url" => message.image_url.clone(),
        "external_id" => message.external_id.clone()
    };
    add_consumer_breadcrumb(
        "postponing image uploading",
        breadcrumb_data,
        ConsumerName::ParseImage,
    );

    produce(&SETTINGS.queue_broker.queues.parse_image, &message).await
}

fn add_consumer_breadcrumb(message: &str, data: BTreeMap<&str, String>, consumer_name: ConsumerName) {
    add_breadcrumb(message, data, &ReportingContext {
        executor: &consumer_name,
        action: "postpone",
    },);
}

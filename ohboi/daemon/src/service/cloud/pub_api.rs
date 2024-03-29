use std::collections::BTreeMap;

use maplit::btreemap;

use lib::error_reporting::{add_breadcrumb, ReportingContext};

use crate::service::cloud::layer::upload_image_to_s3;
use crate::service::Executor;

pub async fn upload_image_to_cloud(file_path: String, image_url: String) -> bool {
    let breadcrumb_data = btreemap! {
        "file_path" => file_path.clone(),
        "image_url" => image_url.clone()
    };
    add_uploader_breadcrumb("uploading image", breadcrumb_data.clone());

    let success = upload_image_to_s3(file_path, image_url).await;

    add_uploader_breadcrumb("uploaded image", breadcrumb_data);

    success
}

fn add_uploader_breadcrumb(message: &str, data: BTreeMap<&str, String>) {
    add_breadcrumb(
        message,
        data,
        &ReportingContext {
            executor: &Executor::Cloud,
            action: "upload",
        },
    );
}

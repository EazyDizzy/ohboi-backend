use maplit::*;
use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3, S3Client, StreamingBody};
use sentry::protocol::map::BTreeMap;

use crate::local_sentry::add_category_breadcrumb;
use crate::parse::service::requester::get_bytes;
use crate::SETTINGS;

pub async fn upload_image_to_cloud(file_path: String, image_url: String) -> bool {
    let breadcrumb_data = btreemap! {
                    "file_path" => file_path.clone(),
                    "image_url" => image_url.clone()
                };
    add_uploader_breadcrumb("downloading image", breadcrumb_data.clone());

    let data = get_bytes(&image_url).await;

    if data.is_err() {
        let message = format!(
            "Cannot get image: {url} {error:?}",
            url = image_url,
            error = data.err().unwrap()
        );
        sentry::capture_message(message.as_str(), sentry::Level::Error);

        return false;
    }

    let client = S3Client::new(Region::EuWest2);
    add_uploader_breadcrumb("uploading image", breadcrumb_data.clone());
    let request: PutObjectRequest = PutObjectRequest {
        bucket: { &SETTINGS.s3.bucket }.to_string(),
        key: file_path,
        // TODO stream directly from http
        body: Some(StreamingBody::from(data.unwrap().to_vec())),
        ..Default::default()
    };

    let result = client.put_object(request).await;
    let success = result.is_ok();

    if !success {
        let message = format!(
            "Image can't be uploaded to cloud! {url} {error:?}",
            url = image_url,
            error = result.err()
        );
        sentry::capture_message(message.as_str(), sentry::Level::Error);
    }
    add_uploader_breadcrumb("uploaded image", breadcrumb_data.clone());

    success
}

pub fn add_uploader_breadcrumb(message: &str, data: BTreeMap<&str, String>) {
    add_category_breadcrumb(message, data, "cloud.upload".into());
}
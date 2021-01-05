use std::env;

use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3, S3Client, StreamingBody};

use crate::parse::requester::get_bytes;

pub async fn upload_image_to_cloud(file_path: String, image_url: String) -> bool {
    let data = get_bytes(image_url.clone()).await;

    if data.is_err() {
        let message = format!(
            "Cannot get image: {} {:?}",
            image_url,
            data.err().unwrap()
        );
        sentry::capture_message(message.as_str(), sentry::Level::Error);

        return false;
    }

    let client = S3Client::new(Region::EuWest2);

    let request: PutObjectRequest = PutObjectRequest {
        bucket: env::var("S3_BUCKET").unwrap(),
        key: file_path,
        // TODO stream directly from http
        body: Some(StreamingBody::from(data.unwrap().to_vec())),
        ..Default::default()
    };

    let result = client.put_object(request).await;
    let success = result.is_ok();

    if !success {
        let message = format!(
            "Image can't be uploaded to cloud! {:?}",
            result.err()
        );
        sentry::capture_message(message.as_str(), sentry::Level::Error);
    }

    success
}
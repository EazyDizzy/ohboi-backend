use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, StreamingBody, S3};

use crate::daemon::service::request::pub_api::get_bytes;
use crate::SETTINGS;

pub async fn upload_image_to_s3(file_path: String, image_url: String) -> bool {
    let data = get_bytes(&image_url).await;

    if data.is_err() {
        let message = format!(
            "[cloud::image::upload] Cannot get image: {url} {error:?}",
            url = image_url,
            error = data.err().unwrap()
        );

        sentry::capture_message(message.as_str(), sentry::Level::Error);

        return false;
    }

    let client = S3Client::new(Region::EuWest2);

    let request: PutObjectRequest = PutObjectRequest {
        bucket: { &SETTINGS.s3.bucket }.to_string(),
        key: file_path,
        // TODO stream directly from http
        body: Some(StreamingBody::from(
            data.expect(&format!("Failed to get bytes from {}", &image_url)),
        )),
        ..Default::default()
    };

    let result = client.put_object(request).await;
    let success = result.is_ok();

    if !success {
        let message = format!(
            "[cloud::image::upload] Image can't be uploaded to cloud! {url} {error:?}",
            url = image_url,
            error = result.err().unwrap()
        );
        sentry::capture_message(message.as_str(), sentry::Level::Error);
    }

    success
}

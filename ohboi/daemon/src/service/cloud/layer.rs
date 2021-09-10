use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, StreamingBody, S3};

use lib::error_reporting;

use crate::service::request::get_bytes;
use crate::SETTINGS;
use lib::error_reporting::ReportingContext;
use crate::service::Executor;

pub async fn upload_image_to_s3(file_path: String, image_url: String) -> bool {
    let data = get_bytes(&image_url).await;
    let context = ReportingContext {
        executor: &Executor::Cloud,
        action: "upload_image"
    };

    if data.is_err() {
        let message = format!(
            "[cloud::image::upload] Cannot get image: {url} {error:?}",
            url = image_url,
            error = data.err().unwrap()
        );

        error_reporting::warning(message.as_str(), &context);

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
        error_reporting::warning(message.as_str(), &context);
    }

    success
}

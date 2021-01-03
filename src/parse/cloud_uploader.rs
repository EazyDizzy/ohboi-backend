use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3, S3Client, StreamingBody};

use crate::parse::requester::get_bytes;

pub async fn upload_image_to_cloud(file_path: String, image_url: &str) -> bool {
    let data = get_bytes(&image_url).await;
    let mut success = data.is_ok();

    if data.is_ok() {
        let client = S3Client::new(Region::EuWest2);

        let request: PutObjectRequest = PutObjectRequest {
            bucket: "ohboi".to_string(),
            key: file_path,
            body: Some(StreamingBody::from(data.unwrap().to_vec())),
            ..Default::default()
        };

        let result = client.put_object(request).await;
        success = result.is_ok();

        if !success {
            println!("s3 upload rror: {:?}", result.err().unwrap());
        }
    }

    success
}
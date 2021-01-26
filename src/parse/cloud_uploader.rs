use lapin::BasicProperties;
use lapin::options::BasicPublishOptions;
use lapin::Result;
use maplit::*;
use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3, S3Client, StreamingBody};
use sentry::{add_breadcrumb, Breadcrumb};
use sentry::protocol::map::BTreeMap;
use sentry::protocol::Value;
use serde::{Deserialize, Serialize};

use crate::parse::queue::get_channel;
use crate::parse::requester::get_bytes;
use crate::SETTINGS;

pub async fn upload_image_to_cloud(file_path: String, image_url: String) -> bool {
    let breadcrumb_data = btreemap! {
                    "file_path" => file_path.clone(),
                    "image_url" => image_url.clone()
                };
    add_uploader_breadcrumb("downloading image", breadcrumb_data.clone());

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
            "Image can't be uploaded to cloud! {:?}",
            result.err()
        );
        sentry::capture_message(message.as_str(), sentry::Level::Error);
    }
    add_uploader_breadcrumb("uploaded image", breadcrumb_data.clone());

    success
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UploadImageMessage {
    pub file_path: String,
    pub image_url: String,
    pub external_id: String,
}

pub async fn upload_image_later(message: UploadImageMessage) -> Result<()> {
    let breadcrumb_data = btreemap! {
                    "file_path" => message.file_path.clone(),
                    "image_url" => message.image_url.clone(),
                    "external_id" => message.external_id.to_string()
                };
    add_uploader_breadcrumb("scheduling later upload", breadcrumb_data);

    let channel = get_channel().await?;

    let payload_json = serde_json::to_string(&message);

    if payload_json.is_err() {
        let message = format!(
            "Can't transform payload to json! {:?} [{:?}]",
            payload_json.err(),
            message
        );
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
        return Ok(());
    }

    let confirm = channel
        .basic_publish(
            "",
            &SETTINGS.amqp.queues.image_upload.name,
            BasicPublishOptions::default(),
            payload_json.unwrap().into_bytes(),
            BasicProperties::default(),
        )
        .await?
        .await?;

    if confirm.is_nack() {
        // TODO what?
        let message = format!(
            "Message is not acknowledged!"
        );
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
    } else {
        log::info!("Message acknowledged");
    }

    Ok(())
}

fn add_uploader_breadcrumb(message: &str, data: BTreeMap<&str, String>) {
    let mut btree_data = BTreeMap::new();

    for pair in data {
        btree_data.insert(pair.0.to_string(), Value::from(pair.1));
    }

    add_breadcrumb(Breadcrumb {
        category: Some("cloud.upload".into()),
        data: btree_data,
        message: Some(message.to_string()),
        ..Default::default()
    });
}
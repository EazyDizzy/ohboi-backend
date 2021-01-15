use futures::StreamExt;
use lapin::{Connection, ConnectionProperties, options::*, Result, types::FieldTable};

pub async fn start() -> Result<()> {
    let address = std::env::var("AMQP_ADDR").expect("AMQP_ADDR should be set");
    let conn = Connection::connect(
        &address,
        ConnectionProperties::default().with_default_executor(8),
    )
        .await?;

    let channel = conn.create_channel().await?;

    let mut consumer = channel
        .basic_consume(
            "crawler_category",
            "crawler_category_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(delivery) = consumer.next().await {
        let (_, delivery) = delivery.expect("error in consumer");

        delivery
            .ack(BasicAckOptions::default())
            .await
            .expect("ack");

        let decoded_data = String::from_utf8(delivery.data);

        if decoded_data.is_err() {
            let message = format!(
                "Can't decode payload to string! {:?}",
                decoded_data.err()
            );
            sentry::capture_message(message.as_str(), sentry::Level::Warning);
            continue;
        }

        println!("Got data: {}", decoded_data.unwrap());
    }

    Ok(())
}
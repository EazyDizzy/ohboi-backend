use lapin::{Channel, Connection, ConnectionProperties, Result};

use crate::SETTINGS;

pub mod consume;
pub mod produce;
pub mod declare;

async fn get_channel() -> Result<Channel> {
    let address = &SETTINGS.queue_broker.url;
    let conn = Connection::connect(address, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    Ok(channel)
}

use lapin::Result;

use crate::queue::layer::produce::produce;
use crate::SETTINGS;

pub async fn start() -> Result<()> {
    produce(&SETTINGS.queue_broker.queues.pull_exchange_rates, "").await
}

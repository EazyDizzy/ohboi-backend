use lapin::Result;
use maplit::btreemap;

use crate::parse::queue::layer::produce::produce;
use crate::parse::queue::producer::add_producer_breadcrumb;
use crate::SETTINGS;

pub async fn start() -> Result<()> {
    add_producer_breadcrumb(
        "creating",
        btreemap! {},
        &SETTINGS.queue_broker.queues.pull_exchange_rates.name,
    );

    produce(&SETTINGS.queue_broker.queues.pull_exchange_rates, "").await
}

use crate::queue::layer::declare::declare_queue;
use crate::SETTINGS;

pub async fn declare_all_queues() {
    let queues = [
        &SETTINGS.queue_broker.queues.parse_category.name,
        &SETTINGS.queue_broker.queues.parse_image.name,
        &SETTINGS.queue_broker.queues.parse_page.name,
        &SETTINGS.queue_broker.queues.pull_exchange_rates.name,
        &SETTINGS.queue_broker.queues.parse_details.name,
    ];

    for queue_name in &queues {
        let declare = declare_queue(queue_name).await;
        if declare.is_err() {
            log::error!("Queue declaration failed. {} {:?}", queue_name, declare);
        }
    }
}

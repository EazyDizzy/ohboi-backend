use crossbeam::channel;
use tokio::runtime::Handle;

use crate::parse::consumer::layer::consume::consume;
use crate::parse::producer::parse_category::ParseCategoryMessage;
use crate::parse::service::parser::parse_category;
use crate::SETTINGS;

pub async fn start() -> core::result::Result<(), ()> {
    let _ = consume(&SETTINGS.amqp.queues.parse_category, |message: String| {
        let (snd, rcv) = channel::bounded(1);

        let _ = Handle::current().spawn(async move {
            let message: ParseCategoryMessage = serde_json::from_str(&message).unwrap();

            let rs = execute(message).await;
            let _ = snd.send(rs);
        });

        rcv.recv().unwrap()
    })
    .await;

    Ok(())
}

async fn execute(message: ParseCategoryMessage) -> Result<(), ()> {
    let parse_result = parse_category(message.source, message.category).await;

    if parse_result.is_err() {
        let message = format!(
            "Parsing failed! [{source}]({category}) {error:?}",
            error = parse_result.err(),
            source = message.source,
            category = message.category
        );
        sentry::capture_message(message.as_str(), sentry::Level::Warning);
        Err(())
    } else {
        Ok(())
    }
}

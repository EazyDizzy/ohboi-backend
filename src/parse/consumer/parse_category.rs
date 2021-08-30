use crossbeam::channel;
use diesel::query_dsl::InternalJoinDsl;
use tokio::runtime::Handle;

use crate::parse::consumer::retrieve_messages;
use crate::parse::producer::parse_category::CrawlerCategoryMessage;
use crate::parse::service::parser::parse_category;
use crate::SETTINGS;

pub async fn start() -> core::result::Result<(), ()> {
    let _ = retrieve_messages(&SETTINGS.amqp.queues.parse_category, |data| {
        let (snd, rcv) = channel::bounded(1);

        let _ = Handle::current().spawn(async move {
            let rs = execute(data).await;
            let a = snd.send(rs);

            println!("send_result {:?}", a);
            rs
        });

        let recieved = rcv.recv();
        println!("recieved result {:?}", recieved);

        recieved.unwrap()
    })
    .await;

    Ok(())
}

async fn execute(data: String) -> Result<(), ()> {
    let parsed_json = serde_json::from_str(&data);
    let message: CrawlerCategoryMessage = parsed_json.unwrap();

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

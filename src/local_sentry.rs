use std::env;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use sentry::{add_breadcrumb, Breadcrumb, ClientInitGuard};
use sentry::protocol::map::BTreeMap;
use sentry::protocol::Value;
use sentry::types::Dsn;
use termion::{color, style};

pub fn init_sentry() -> ClientInitGuard {
    std::env::set_var("RUST_BACKTRACE", "1");

    sentry::init(
        sentry::ClientOptions {
            dsn: Some(
                Dsn::from_str(dotenv::var("SENTRY_DSN").expect("SENTRY_DSN should be set").as_str())
                    .unwrap()
            ),
            attach_stacktrace: true,
            send_default_pii: true,
            auto_session_tracking: true,
            release: Some(env::var("CARGO_PKG_VERSION").or_else::<String, _>(|_| {
                Ok(String::from("unknown"))
            }).unwrap().into()),
            shutdown_timeout: Duration::from_secs(10),

            before_send: Some(Arc::new(|event| {
                if event.message.is_some() {
                    log::error!(
                        "{}{:#?}{}",
                        color::Fg(color::Red),
                        event.message.clone().unwrap(),
                        style::Reset
                    );
                }

                Some(event)
            })),
            ..Default::default()
        }
    )
}

pub fn add_category_breadcrumb(message: &str, data: BTreeMap<&str, String>, category: String) {
    let mut btree_data = BTreeMap::new();

    for pair in data {
        btree_data.insert(pair.0.to_string(), Value::from(pair.1));
    }
    let breadcrumb = Breadcrumb {
        category: Some(category),
        data: btree_data,
        message: Some(message.to_string()),
        ..Default::default()
    };

    add_breadcrumb(breadcrumb.clone());

    if cfg!(debug_assertions) {
        log::info!(
            "{}{}{} {}{}{} {}{:?}{}",
            color::Fg(color::Magenta),
            breadcrumb.category.clone().unwrap(),
            style::Reset,
            //
            color::Fg(color::Yellow),
            breadcrumb.message.clone().unwrap(),
            style::Reset,
            //
            color::Fg(color::LightBlue),
            breadcrumb.data,
            style::Reset,
        );
    } else {
        log::info!(
            "{} {} {:?}",
            breadcrumb.category.clone().unwrap(),
            breadcrumb.message.clone().unwrap(),
            breadcrumb.data,
        );
    }
}
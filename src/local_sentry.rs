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
                        "{c}{message:#?}{c_end}",
                        c = color::Fg(color::Red),
                        message = event.message.clone().unwrap(),
                        c_end = style::Reset
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
            "{c1}{category}{r} {c2}{message}{r} {c3}{data:?}{r}",
            c1 = color::Fg(color::Magenta),
            c2 = color::Fg(color::Yellow),
            c3 = color::Fg(color::LightBlue),
            r = style::Reset,
            category = breadcrumb.category.clone().unwrap(),
            message = breadcrumb.message.clone().unwrap(),
            data = breadcrumb.data,
        );
    } else {
        log::info!(
            "{category} {message} {data:?}",
            category = breadcrumb.category.clone().unwrap(),
            message = breadcrumb.message.clone().unwrap(),
            data = breadcrumb.data,
        );
    }
}
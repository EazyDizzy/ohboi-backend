use std::env;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use sentry::types::Dsn;
use sentry::ClientInitGuard;
pub use sentry::*;
use termion::{color, style};

pub fn init() -> ClientInitGuard {
    std::env::set_var("RUST_BACKTRACE", "1");

    sentry::init(sentry::ClientOptions {
        dsn: Some(
            Dsn::from_str(
                dotenv::var("SENTRY_DSN")
                    .expect("SENTRY_DSN should be set")
                    .as_str(),
            )
            .unwrap(),
        ),
        attach_stacktrace: true,
        send_default_pii: true,
        auto_session_tracking: true,
        release: Some(
            env::var("CARGO_PKG_VERSION")
                .or_else::<String, _>(|_| Ok(String::from("unknown")))
                .unwrap()
                .into(),
        ),
        shutdown_timeout: Duration::from_secs(10),

        before_send: Some(Arc::new(|event| {
            if event.message.is_some() {
                log::error!(
                    "{c}{message:#?}{c_end}",
                    c = color::Fg(color::Red),
                    message = event.message.as_ref().unwrap(),
                    c_end = style::Reset
                );
            }

            // Some(event)
            None
        })),
        ..Default::default()
    })
}

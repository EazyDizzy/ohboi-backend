use std::env;
use std::sync::Arc;

use sentry::ClientInitGuard;
use termion::{color, style};

pub fn init_sentry() -> ClientInitGuard {
    sentry::init(
        sentry::ClientOptions {
            attach_stacktrace: true,
            send_default_pii: true,
            auto_session_tracking: true,
            release: Some(env::var("CARGO_PKG_VERSION").unwrap().into()),

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
            before_breadcrumb: Some(Arc::new(|breadcrumb| {
                if breadcrumb.message.is_some() {
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
                }

                Some(breadcrumb)
            })),
            ..Default::default()
        }
    )
}
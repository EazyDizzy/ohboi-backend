use std::collections::BTreeMap;

use termion::{color, style};

use crate::error_reporting::ReportingContext;

#[allow(dead_code)]
pub fn add_breadcrumb(message: &str, data: BTreeMap<&str, String>, context: &ReportingContext) {
    let mut btree_data = BTreeMap::new();
    for pair in data {
        btree_data.insert(pair.0.to_string(), sentry::protocol::Value::from(pair.1));
    }

    let breadcrumb = sentry::Breadcrumb {
        category: Some(context.to_string()),
        data: btree_data,
        message: Some(message.to_string()),
        ..Default::default()
    };

    if cfg!(debug_assertions) {
        log::info!(
            "{c1}{category}{r} {c2}{message}{r} {c3}{data:?}{r}",
            c1 = color::Fg(color::Magenta),
            c2 = color::Fg(color::Yellow),
            c3 = color::Fg(color::LightBlue),
            r = style::Reset,
            category = breadcrumb.category.as_ref().unwrap(),
            message = breadcrumb.message.as_ref().unwrap(),
            data = &breadcrumb.data,
        );
    } else {
        log::info!(
            "{category} {message} {data:?}",
            category = breadcrumb.category.as_ref().unwrap(),
            message = breadcrumb.message.as_ref().unwrap(),
            data = &breadcrumb.data,
        );
    }

    sentry::add_breadcrumb(breadcrumb);
}

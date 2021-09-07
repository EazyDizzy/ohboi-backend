use crate::error_reporting::ReportingContext;

pub fn fatal(message: &str, context: &ReportingContext) {
    report(message, context, sentry::Level::Fatal);
}
pub fn error(message: &str, context: &ReportingContext) {
    report(message, context, sentry::Level::Error);
}
pub fn warning(message: &str, context: &ReportingContext) {
    report(message, context, sentry::Level::Warning);
}
pub fn info(message: &str, context: &ReportingContext) {
    report(message, context, sentry::Level::Info);
}

fn report(message: &str, context: &ReportingContext, lvl: sentry::Level) {
    let prefix = context.to_string();
    sentry::capture_message(&[&prefix, message].concat(), lvl);
}

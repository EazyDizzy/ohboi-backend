pub use breadcrumb::add_breadcrumb;
pub use init::init;
pub use report::*;

mod breadcrumb;
mod init;
mod report;

pub trait DisplayString: std::fmt::Debug {
    fn to_display_string(&self) -> String;
}

pub struct ReportingContext<'root> {
    pub executor: &'root dyn DisplayString,
    pub action: &'root str,
}

impl ToString for ReportingContext<'_> {
    fn to_string(&self) -> String {
        format!("[{:?}::{:?}] ", self.executor, self.action)
    }
}

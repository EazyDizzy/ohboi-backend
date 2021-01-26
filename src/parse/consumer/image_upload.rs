use futures::StreamExt;
use lapin::{options::*, Result, types::FieldTable};
use maplit::*;
use sentry::{add_breadcrumb, Breadcrumb};
use sentry::protocol::map::BTreeMap;
use sentry::protocol::Value;

use crate::parse::crawler::mi_shop_com::MiShopComCrawler;
use crate::parse::db::entity::SourceName;
use crate::parse::parser::parse;
use crate::parse::producer::crawler_category::CrawlerCategoryMessage;
use crate::parse::queue::get_channel;
use crate::SETTINGS;

pub async fn start() -> Result<()> {
    Ok(())
}
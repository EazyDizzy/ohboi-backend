use crate::daemon::db::entity::source::SourceName;
use crate::daemon::parse::crawler::crawler::Crawler;
use crate::daemon::parse::crawler::samsung_shop_com_ua::SamsungShopComUaCrawler;
use crate::daemon::parse::crawler::mi_shop_com::crawler::MiShopComCrawler;

pub mod crawler;
pub mod mi_shop_com;
pub mod samsung_shop_com_ua;
mod characteristic_parser;

pub fn get_crawler(source: &SourceName) -> &dyn Crawler {
    match source {
        SourceName::MiShopCom => &MiShopComCrawler {},
        SourceName::SamsungShopComUa => &SamsungShopComUaCrawler {},
    }
}
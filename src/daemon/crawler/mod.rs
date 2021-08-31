use crate::daemon::db::entity::source::SourceName;
use crate::daemon::crawler::crawler::Crawler;
use crate::daemon::crawler::mi_shop_com::MiShopComCrawler;
use crate::daemon::crawler::samsung_shop_com_ua::SamsungShopComUaCrawler;

pub mod crawler;
pub mod mi_shop_com;
pub mod samsung_shop_com_ua;
mod util;

pub fn get_crawler(source: &SourceName) -> &dyn Crawler {
    match source {
        SourceName::MiShopCom => &MiShopComCrawler {},
        SourceName::SamsungShopComUa => &SamsungShopComUaCrawler {},
    }
}
pub use crawler::*;
pub use mi_shop_com::crawler::MiShopComCrawler;
pub use samsung_shop_com_ua::SamsungShopComUaCrawler;
pub use samsung_shop_com_ua::*;

use crate::db::entity::source::SourceName;

mod characteristic_parser;
mod crawler;
mod mi_shop_com;
mod samsung_shop_com_ua;

pub fn get_crawler(source: &SourceName) -> &dyn Crawler {
    match source {
        SourceName::MiShopCom => &MiShopComCrawler {},
        SourceName::SamsungShopComUa => &SamsungShopComUaCrawler {},
    }
}

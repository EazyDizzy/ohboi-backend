use std::fmt;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use lib::my_enum::CurrencyEnum;

#[derive(Queryable, Debug)]
pub struct Source {
    pub id: i32,
    pub site_name: String,
    pub logo: String,
    pub currency: CurrencyEnum,
    pub enabled: bool,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum SourceName {
    MiShopCom,
    SamsungShopComUa,
}

impl fmt::Display for SourceName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
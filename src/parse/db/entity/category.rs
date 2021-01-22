use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
pub struct Category {
    pub id: i32,
    pub slug: String,
    pub parent_id: Option<i32>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum CategorySlug {
    SmartHome,
    Smartphone,
    Headphones,
    Watches,
}

impl fmt::Display for CategorySlug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
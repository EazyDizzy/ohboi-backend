use serde::{Serialize};
use std::fmt;

#[derive(Serialize, Queryable)]
pub struct Category {
    pub id: i32,
    pub slug: String,
    pub parent_od: i32,
}

#[derive(Debug, Serialize)]
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
use serde::{Serialize};

#[derive(Serialize, Queryable)]
pub struct Category {
    pub id: i32,
    pub title: String,
    pub parent_od: i32,
}
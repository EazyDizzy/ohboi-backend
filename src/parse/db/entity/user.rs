use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,

    #[serde(skip)]
    pub created_at: NaiveDateTime,
    #[serde(skip)]
    pub updated_at: NaiveDateTime,
}
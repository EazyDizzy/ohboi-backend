use chrono::NaiveDateTime;
use serde::Serialize;

use lib::schema::users;

#[derive(Serialize, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,

    #[serde(skip)]
    pub created_at: NaiveDateTime,
    #[serde(skip)]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub created_at: &'a NaiveDateTime,
    pub updated_at: &'a NaiveDateTime,
}
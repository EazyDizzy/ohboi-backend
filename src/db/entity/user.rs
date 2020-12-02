#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

use super::schema::users;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
}
use crate::http::db;
use crate::http::db::entity::Source;
use crate::diesel::prelude::*;

pub fn get_all_enabled() -> Vec<Source> {
    use crate::schema::source::dsl::*;
    let connection = &db::establish_connection();

    source
        .filter(enabled.eq(true))
        .load(connection)
        .expect("Cannot load sources")
}
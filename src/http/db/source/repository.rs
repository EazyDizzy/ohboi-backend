use diesel::prelude::*;

use crate::common::db;
use crate::http::db::source::entity::Source;

pub fn get_all_enabled() -> Vec<Source> {
    use crate::schema::source::dsl::*;
    let connection = &db::establish_connection();

    source
        .filter(enabled.eq(true))
        .load(connection)
        .expect("Cannot load sources")
}
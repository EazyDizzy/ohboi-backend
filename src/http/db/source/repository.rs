use lib::diesel::prelude::*;

use lib::db;
use crate::http::db::source::entity::Source;

pub fn get_all_enabled() -> Vec<Source> {
    use lib::schema::source::dsl::{enabled, source};
    let connection = &db::establish_connection();

    source
        .filter(enabled.eq(true))
        .load(connection)
        .expect("Cannot load sources")
}
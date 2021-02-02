use inflector::Inflector;

use crate::diesel::prelude::*;
use crate::parse::db;
use crate::parse::db::entity::{Source, SourceName};

pub fn get_source(name: &SourceName) -> Source {
    use crate::schema::source::dsl::*;
    let connection = &db::establish_connection();

    let filter = site_name.eq(name.to_string().to_snake_case());

    let results: Vec<Source> = source.filter(filter)
                                     .limit(1)
                                     .load::<Source>(connection)
                                     .expect("Cannot load source");

    results.into_iter().next().unwrap()
}
// TODO separate lib dependencies from crates
#[macro_use]
pub extern crate diesel;
#[macro_use]
extern crate lazy_static;

use std::env;

use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, r2d2};

pub mod db;
pub mod dto;
pub mod my_enum;
pub mod schema;
pub mod service;
pub mod util;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: Pool = {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool")
    };
}

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

use crate::diesel::types::Varchar;
use crate::POOL;

pub mod entity;
pub mod repository;

pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    POOL.get().unwrap()
}

sql_function!(fn lower(x: Varchar) -> Varchar);

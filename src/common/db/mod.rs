use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};

use crate::POOL;

pub mod entity;
pub mod repository;

pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    POOL.get().unwrap()
}
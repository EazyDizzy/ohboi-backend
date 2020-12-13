pub mod repository;
pub mod entity;

use r2d2::{PooledConnection};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;

use crate::POOL;

pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    POOL.get().unwrap()
}
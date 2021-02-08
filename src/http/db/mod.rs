use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;

use crate::POOL;

pub mod repository;
pub mod entity;
pub mod product;
pub mod category;
pub mod source;

pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    POOL.get().unwrap()
}
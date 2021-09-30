use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};

use crate::diesel::types::Varchar;
use crate::POOL;

pub mod entity;
pub mod repository;
pub mod characteristic;
pub mod product_characteristic;

pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    POOL.get().unwrap()
}

sql_function!(fn lower(x: Varchar) -> Varchar);

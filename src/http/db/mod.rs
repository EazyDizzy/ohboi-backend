use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::types::Varchar;
use r2d2::PooledConnection;

use crate::POOL;

pub mod product;
pub mod category;
pub mod source;
pub mod source_product;
pub mod user;
pub mod user_registration;

pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    POOL.get().unwrap()
}

sql_function!(fn lower(x: Varchar) -> Varchar);

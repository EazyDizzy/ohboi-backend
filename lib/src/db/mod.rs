use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};

use crate::diesel::types::Varchar;
use crate::POOL;

pub mod characteristic;
pub mod product_characteristic;
pub mod exchange_rate;
pub mod source_product_price_history;

pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    POOL.get().unwrap()
}

sql_function!(fn lower(x: Varchar) -> Varchar);

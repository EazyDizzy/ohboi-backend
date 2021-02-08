use diesel::sql_types::Varchar;

pub use source_product::*;
pub use source_product_price_history::*;
pub use user::*;
pub use user_registration::*;

pub use crate::http::db::product::entity::*;

mod user;
mod source_product;
mod source_product_price_history;
pub mod user_registration;

sql_function!(fn lower(x: Varchar) -> Varchar);

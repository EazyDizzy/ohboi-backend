use diesel::sql_types::Varchar;

pub use category::*;
pub use product::*;
pub use source::*;
pub use source_product::*;
pub use source_product_price_history::*;
pub use user::*;
pub use user_registration::*;

mod user;
mod product;
mod category;
mod source;
mod source_product;
mod source_product_price_history;
pub mod user_registration;

sql_function!(fn lower(x: Varchar) -> Varchar);

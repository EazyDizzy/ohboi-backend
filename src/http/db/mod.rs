use diesel::types::Varchar;

pub mod product;
pub mod category;
pub mod source;
pub mod source_product;
pub mod user;
pub mod user_registration;
pub mod product_characteristic;

sql_function!(fn lower(x: Varchar) -> Varchar);

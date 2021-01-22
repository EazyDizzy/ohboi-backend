table! {
    use diesel::sql_types::*;
    

    category (id) {
        id -> Int4,
        slug -> Varchar,
        parent_id -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;
    

    product (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        lowest_price -> Numeric,
        images -> Nullable<Array<Varchar>>,
        category -> Int4,
        enabled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    

    source (id) {
        id -> Int4,
        site_name -> Varchar,
        logo -> Varchar,
        enabled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    

    source_product (id) {
        id -> Int4,
        source_id -> Int4,
        product_id -> Int4,
        external_id -> Varchar,
        price -> Numeric,
        enabled -> Bool,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    

    source_product_price_history (id) {
        id -> Int4,
        source_id -> Int4,
        product_id -> Int4,
        price -> Numeric,
        external_id -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::my_enum::*;

    user_registration (id) {
        id -> Int4,
        user_id -> Int4,
        registration_type -> User_registration_type,
        email -> Varchar,
        full_name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    

    users (id) {
        id -> Int4,
        username -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(product -> category (category));
joinable!(source_product -> product (product_id));
joinable!(source_product -> source (source_id));
joinable!(source_product_price_history -> product (product_id));
joinable!(source_product_price_history -> source (source_id));
joinable!(user_registration -> users (user_id));

allow_tables_to_appear_in_same_query!(
    category,
    product,
    source,
    source_product,
    source_product_price_history,
    user_registration,
    users,
);

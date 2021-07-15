table! {
    use diesel::sql_types::*;
    use crate::my_enum::*;

    category (id) {
        id -> Int4,
        slug -> Varchar,
        parent_id -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::my_enum::*;

    category_characteristic (id) {
        id -> Int4,
        category_id -> Int4,
        characteristic_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::my_enum::*;

    characteristic (id) {
        id -> Int4,
        slug -> Varchar,
        enabled -> Bool,
        visualisation_type -> Characteristic_visualisation_type,
        value_type -> Characteristic_value_type,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::my_enum::*;

    exchange_rate (id) {
        id -> Int4,
        currency -> Currency_enum,
        rate -> Numeric,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::my_enum::*;

    product (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        lowest_price -> Numeric,
        highest_price -> Numeric,
        images -> Nullable<Array<Varchar>>,
        category -> Int4,
        enabled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::my_enum::*;

    product_characteristic (id) {
        id -> Int4,
        product_id -> Int4,
        characteristic_id -> Int4,
        value_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::my_enum::*;

    product_characteristic_enum_value (id) {
        id -> Int4,
        value -> Int2,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::my_enum::*;

    product_characteristic_float_value (id) {
        id -> Int4,
        value -> Float8,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::my_enum::*;

    product_characteristic_int_value (id) {
        id -> Int4,
        value -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::my_enum::*;

    product_characteristic_string_value (id) {
        id -> Int4,
        value -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::my_enum::*;

    source (id) {
        id -> Int4,
        site_name -> Varchar,
        logo -> Varchar,
        currency -> Currency_enum,
        enabled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::my_enum::*;

    source_product (id) {
        id -> Int4,
        source_id -> Int4,
        product_id -> Int4,
        external_id -> Varchar,
        original_price -> Numeric,
        price -> Numeric,
        enabled -> Bool,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::my_enum::*;

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
    use crate::my_enum::*;

    users (id) {
        id -> Int4,
        username -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(category_characteristic -> category (category_id));
joinable!(category_characteristic -> characteristic (characteristic_id));
joinable!(product -> category (category));
joinable!(product_characteristic -> characteristic (characteristic_id));
joinable!(product_characteristic -> product (product_id));
joinable!(source_product -> product (product_id));
joinable!(source_product -> source (source_id));
joinable!(source_product_price_history -> product (product_id));
joinable!(source_product_price_history -> source (source_id));
joinable!(user_registration -> users (user_id));

allow_tables_to_appear_in_same_query!(
    category,
    category_characteristic,
    characteristic,
    exchange_rate,
    product,
    product_characteristic,
    product_characteristic_enum_value,
    product_characteristic_float_value,
    product_characteristic_int_value,
    product_characteristic_string_value,
    source,
    source_product,
    source_product_price_history,
    user_registration,
    users,
);

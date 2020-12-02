table! {
    product (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        lowest_price -> Numeric,
        images -> Array<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    source (id) {
        id -> Int4,
        site_name -> Varchar,
        id_regex -> Varchar,
        name_selector -> Varchar,
        price_selector -> Varchar,
        logo -> Varchar,
        enabled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    source_product (id) {
        id -> Int4,
        source_id -> Int4,
        product_id -> Int4,
        price -> Numeric,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(source_product -> product (product_id));
joinable!(source_product -> source (source_id));

allow_tables_to_appear_in_same_query!(
    product,
    source,
    source_product,
    users,
);

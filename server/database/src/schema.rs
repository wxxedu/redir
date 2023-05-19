// @generated automatically by Diesel CLI.

diesel::table! {
    accesses (id) {
        id -> Nullable<Integer>,
        url_id -> Integer,
        accessed_at -> Timestamp,
        ip -> Text,
    }
}

diesel::table! {
    urls (id) {
        id -> Nullable<Integer>,
        url -> Text,
        created_at -> Timestamp,
        num_accesses -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(accesses -> urls (url_id));

diesel::allow_tables_to_appear_in_same_query!(
    accesses,
    urls,
    users,
);

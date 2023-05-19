// @generated automatically by Diesel CLI.

diesel::table! {
    accesses (id) {
        id -> Integer,
        url_id -> Integer,
        accessed_at -> Timestamp,
        ip -> Text,
    }
}

diesel::table! {
    urls (id) {
        id -> Integer,
        url -> Text,
        created_at -> Timestamp,
        num_accesses -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        created_at -> Timestamp,
        salt -> Text,
    }
}

diesel::joinable!(accesses -> urls (url_id));

diesel::allow_tables_to_appear_in_same_query!(accesses, urls, users,);

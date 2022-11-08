// @generated automatically by Diesel CLI.

diesel::table! {
    movies (id) {
        id -> Integer,
        title -> Text,
        watched_at -> Timestamp,
        created_at -> Timestamp,
    }
}

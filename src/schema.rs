// @generated automatically by Diesel CLI.

diesel::table! {
    movies (id) {
        id -> Integer,
        title -> Text,
        created_at -> Timestamp,
    }
}

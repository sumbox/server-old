// @generated automatically by Diesel CLI.

diesel::table! {
    ideas (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

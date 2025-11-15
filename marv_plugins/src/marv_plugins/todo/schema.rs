// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        body -> Text,
        status -> Varchar,
    }
}

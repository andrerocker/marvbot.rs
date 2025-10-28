// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    todos (id) {
        id -> Int4,
        body -> Text,
        status -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(messages, todos,);

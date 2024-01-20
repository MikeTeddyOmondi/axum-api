// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        public_id -> Text,
        title -> Text,
        description -> Text,
        completed -> Bool,
    }
}

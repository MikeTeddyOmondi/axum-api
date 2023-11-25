// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        public_id -> Text,
        title -> Text,
        description -> Text,
        completed -> Integer,
    }
}

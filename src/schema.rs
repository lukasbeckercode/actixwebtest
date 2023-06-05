// @generated automatically by Diesel CLI.

diesel::table! {
    parts (id) {
        id -> Int4,
        description -> Text,
        num_expected -> Int4,
        num_actual -> Int4,
    }
}

// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
    }
}

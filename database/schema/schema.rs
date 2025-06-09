// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        email -> Nullable<Text>,
    }
}

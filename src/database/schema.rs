// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        message -> Text,
        published -> Bool,
    }
}

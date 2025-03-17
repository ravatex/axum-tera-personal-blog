// @generated automatically by Diesel CLI.

diesel::table! {
    inquiries (id) {
        id -> Integer,
        name -> Text,
        message -> Text,
        email -> Text,
        business -> Bool,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        name -> Text,
        date -> Date,
        message -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    inquiries,
    posts,
);

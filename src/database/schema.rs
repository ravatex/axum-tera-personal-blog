// @generated automatically by Diesel CLI.

diesel::table! {
    comments (post_id, id) {
        post_id -> Integer,
        id -> Integer,
        name -> Text,
        date -> Date,
        contents -> Text,
    }
}

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
        thumbnail -> Nullable<Text>,
    }
}

diesel::table! {
    thumbnails (filepath) {
        filepath -> Text,
        alt_text -> Nullable<Text>,
    }
}

diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(posts -> thumbnails (thumbnail));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    inquiries,
    posts,
    thumbnails,
);

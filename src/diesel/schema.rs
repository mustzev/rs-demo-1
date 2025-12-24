// @generated automatically by Diesel CLI.

diesel::table! {
    demos (id) {
        col1 -> Nullable<Text>,
        id -> Int4,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(demos, posts,);

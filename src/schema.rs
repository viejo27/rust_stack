// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        last_name -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    user,
);

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        category -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        user_type -> Varchar,
        udid -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);

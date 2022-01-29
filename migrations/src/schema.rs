table! {
    drafts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        link -> Text,
        published -> Bool,
    }
}

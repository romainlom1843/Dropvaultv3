table! {
    tokens (token_id) {
        token_id -> Int4,
        login -> Text,
        token -> Text,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        hashe -> Text,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    tokens,
    users,
);

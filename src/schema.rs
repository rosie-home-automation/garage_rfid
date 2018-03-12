table! {
    credentials (id) {
        id -> Text,
        user_id -> Text,
        name -> Text,
        variety -> Text,
        value -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Text,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    credentials,
    users,
);

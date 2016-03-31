table! {
    access_tokens {
        id -> BigSerial,
        user_id -> Text,
        value -> Text,
        created_at -> Timestamp,
    }
}

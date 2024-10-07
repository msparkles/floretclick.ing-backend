// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        #[max_length = 256]
        id -> Varchar,
        metadata -> Jsonb,
    }
}

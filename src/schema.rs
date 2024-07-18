// @generated automatically by Diesel CLI.

diesel::table! {
    servers (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        ip -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        pw_hash -> Bytea,
        #[max_length = 255]
        username -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    servers,
    users,
);

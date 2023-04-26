// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Int4,
        email -> Varchar,
        password_hash -> Bytea,
    }
}

diesel::table! {
    message (id) {
        id -> Int4,
        author -> Int4,
        date -> Timestamp,
        content -> Varchar,
    }
}

diesel::table! {
    read (id) {
        id -> Int4,
        account -> Int4,
        message -> Int4,
    }
}

diesel::joinable!(message -> account (author));
diesel::joinable!(read -> account (account));
diesel::joinable!(read -> message (message));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    message,
    read,
);

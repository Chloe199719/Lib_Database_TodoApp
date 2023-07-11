// @generated automatically by Diesel CLI.

diesel::table! {
    email_verify_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        token -> Varchar,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    sessions (id) {
        id -> Uuid,
        #[max_length = 255]
        token -> Varchar,
        user_id -> Uuid,
        expires_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    todos (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        completed -> Bool,
        todopriority -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        email_verified -> Bool,
    }
}

diesel::joinable!(email_verify_tokens -> users (user_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(todos -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    email_verify_tokens,
    sessions,
    todos,
    users,
);

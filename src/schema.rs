// @generated automatically by Diesel CLI.

diesel::table! {
    file (id) {
        id -> Nullable<Integer>,
        file_name -> Text,
        hashed_file_name -> Text,
        content_hash -> Text,
        content_type -> Text,
        size -> Integer,
        storage_path -> Text,
        owner_id -> Nullable<Integer>,
        is_public -> Nullable<Integer>,
        is_deleted -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    file_to_link (id) {
        id -> Nullable<Integer>,
        link -> Nullable<Text>,
        filename -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        name -> Text,
        email -> Text,
        password -> Text,
    }
}

diesel::joinable!(file -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(
    file,
    file_to_link,
    users,
);

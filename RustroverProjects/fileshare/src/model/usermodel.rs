use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc}; // IMPORTANT: Use DateTime<Utc>
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use crate::schema::*; // Make sure this brings in `file` and `users` tables

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct User{
    // Make sure these match schema.rs::users table types and order
    pub id: Option<i32>, // Assuming users.id -> Nullable<Integer>
    pub name: String,    // Assuming users.name -> Text
    pub password: String,// Assuming users.password -> Text
    pub email: String,   // Assuming users.email -> Text
}
#[derive(Insertable)]
#[derive(Deserialize, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CreateUserRequest{
    pub name: String,
    pub password: String,
    pub email: String,
}

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::file)] // Path seems correct
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct File {
    pub id: Option<i32>, // `id -> Nullable<Integer>`
    pub file_name: String,
    pub hashed_file_name: String,
    pub content_hash: String,
    pub content_type: String,
    pub size: i32, // `size -> Integer`
    pub storage_path: String,
    pub owner_id: Option<i32>, // `owner_id -> Nullable<Integer>`
    pub is_public: Option<i32>, // `is_public -> Nullable<Integer>`
    pub is_deleted: Option<i32>, // `is_deleted -> Nullable<Integer>`
    pub created_at: Option<NaiveDateTime>, // <--- THIS IS THE FIX
    pub updated_at: Option<NaiveDateTime>, // <--- THIS IS THE FIX
    pub deleted_at: Option<NaiveDateTime>, // <--- THIS IS THE FIX
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = file)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct FileToInsert {
    pub file_name: String,
    pub hashed_file_name: String,
    pub content_hash: String,
    pub content_type: String,
    pub size: i32,
    pub storage_path: String,
    pub owner_id: Option<i32>,
    pub is_public: Option<i32>,
    pub is_deleted: Option<i32>,
    // Timestamps are omitted here because your SQL schema has DEFAULT CURRENT_TIMESTAMP for them,
    // so Diesel will not try to insert them, relying on the DB to set them.
}
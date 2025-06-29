use axum::response;

use std::env::VarError;
use std::fmt;
use std::fmt::Formatter;
use std::io::Error;
use std::num::TryFromIntError;
use aws_sdk_s3::config::http::HttpResponse;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::get_object::{GetObject, GetObjectError, GetObjectOutput};
use axum::extract::multipart::MultipartError;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use bcrypt::BcryptError;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use tokio::task::JoinError;
use crate::schema::*;
use crate::schema::users::email;

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
#[derive(Serialize,Deserialize,Queryable, Clone)]
pub struct LoginRequest{
    pub name:String,
    pub password: String,
    pub email: String
}

pub struct LoginResponse{
    pub status_code: StatusCode,
    pub jwt_token: String
}

impl IntoResponse for LoginResponse{
    fn into_response(self) -> Response {
        let res_json = serde_json::json!({
            "token" : self.jwt_token,
        });
        (StatusCode::OK, Json(res_json)).into_response()
    }
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

#[derive(Insertable, Deserialize, Serialize, Debug)]
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

#[derive(Debug)]
pub enum ConversionError {
    ConversionError(String)
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConversionError::ConversionError(message) => write!(f,"Conversion Error {} ", message)
        }
    }
}

impl std::error::Error for ConversionError{

}
impl From<TryFromIntError> for ConversionError{
    fn from(value: TryFromIntError) -> Self {
        ConversionError::ConversionError(format!("Could nor convert: {} ", value))
    }
}

impl From<BcryptError> for ConversionError{
    fn from(value: BcryptError) -> Self {
        ConversionError::ConversionError(format!("Error Message:{}", value))
    }
}
impl IntoResponse for ConversionError{
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro with Storing File and Provide Link: {}", self)).into_response()
    }
}
impl From<MultipartError> for ConversionError {
    fn from(err: MultipartError) -> Self {
        ConversionError::ConversionError("Erorr".to_string())
    }
}
impl From<VarError> for ConversionError{
    fn from(value: VarError) -> Self {
        ConversionError::ConversionError("Error Converting stuff".to_string())
    }
}
impl From<JoinError> for ConversionError{
    fn from(value: JoinError) -> Self {
        println!("{}", value);
        ConversionError::ConversionError("Error Join Handle".to_string())
    }
}

impl From<Box<dyn std::error::Error>> for ConversionError{
    fn from(value: Box<dyn std::error::Error>) -> Self {
        ConversionError::ConversionError("Error".to_string())
    }
}

impl From<aws_smithy_runtime_api::client::result::SdkError<GetObjectError, aws_smithy_runtime_api::http::Response>> for ConversionError{
    fn from(value: SdkError<GetObjectError, aws_smithy_runtime_api::http::Response>) -> Self {
        ConversionError::ConversionError("Error with Download AWS SDK".to_string())
    }
}
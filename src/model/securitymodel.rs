use std::fmt;
use std::fmt::{write, Formatter};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use crate::model::usermodel;


#[derive(Deserialize, Serialize, Clone)]
pub struct EncodeJWT{
    pub(crate) username: String,
    pub(crate) email: String
}

#[derive(Debug)]
pub enum AuthError {
    AuthError(String,StatusCode)
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::AuthError(message, status) => write!(f,"Error: {},StatusCode: {}", message, status)
        }
        
    }
}


impl std::error::Error for AuthError{
}

impl From<usermodel::ConversionError> for AuthError {
    fn from(err: usermodel::ConversionError) -> Self {
        
        AuthError::AuthError("Error".to_string(), StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl IntoResponse for AuthError{
    fn into_response(self) -> Response {
        StatusCode::FORBIDDEN.into_response()
    }
}
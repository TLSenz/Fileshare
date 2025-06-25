use std::fmt;
use std::fmt::{write, Formatter};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::model::usermodel::ConversionError;

#[derive(Deserialize, Serialize)]
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
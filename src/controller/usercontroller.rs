use std::fmt::format;
use std::io::Error;
use std::string::String;
use std::fs::File;
use std::io::Write;
use axum::http::{StatusCode};
use axum::{ Json};
use axum::response::IntoResponse;
use crate::model::usermodel::{ConversionError, CreateUserRequest};
use crate::service::userservice::{create_user, store_files};
use axum::extract::{Multipart};
use crate::model::usermodel::ConversionError::*;

// #[axum::debug_handler]
pub async fn signup(Json(user):Json<CreateUserRequest> ) -> impl IntoResponse{
    
    let result =   create_user(user).await;
    
    if result == true{
        StatusCode::OK
    }
    else { 
        StatusCode::CONFLICT
    }
}

pub async fn upload_file(mut file: Multipart) -> Result<String,ConversionError>{

    let is_stored = store_files(file).await;
   match is_stored {
        Ok(links) => {
           
            if let Some(first_link) = links.into_iter().next() {
                Ok(first_link)
            } else {
                
                Err(ConversionError("error".to_string()))
            }
        }
        Err(error) => {
            
            Err(error)
        }
    }
    


}


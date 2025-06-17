use std::string::String;
use std::fs::File;
use std::io::Write;
use axum::http::{StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use crate::model::usermodel::{CreateUserRequest};
use crate::service::userservice::{create_user, store_files};
use axum::extract::{Multipart};

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

pub async fn upload_file(mut file: Multipart) -> impl IntoResponse{
    
   let is_stored = store_files(file).await;
    

    

}
use axum::http::{StatusCode};
use axum::{ Json};
use axum::response::IntoResponse;
use crate::model::usermodel::{CreateUserRequest, LoginRequest};
use crate::service::userservice::{create_user};

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

pub async fn login(Json(user):Json<LoginRequest>) -> impl IntoResponse{
    
}


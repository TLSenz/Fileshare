use std::sync::Arc;
use axum::http::{HeaderMap, StatusCode};
use axum::{Extension, Json};
use axum::response::IntoResponse;
use crate::model::securitymodel::{AuthError, EncodeJWT};
use crate::model::usermodel::{ConversionError, CreateUserRequest, LoginRequest, LoginResponse};
use crate::Security::jwt::encode_jwt;
use crate::service::fileservice::create_folder_for_user;
use crate::service::userservice::{check_user_login, create_user};

#[axum::debug_handler]
pub async fn signup(Json(user):Json<CreateUserRequest>) -> Result<(), ConversionError>{
    
    let result =   create_user(user).await?;
    
    
    if result == true{
       Ok(())
    }
    else { 
        Err(ConversionError::ConversionError("Auathenticated".to_string()))
    }
}

pub async fn login(Json(user):Json<LoginRequest>) -> Result<LoginResponse, AuthError>{
    
    if check_user_login(user.clone()).await?{
        let token = encode_jwt(&user.name, user.email.as_str())?;
        
        let response = LoginResponse{
            status_code: StatusCode::OK,
            jwt_token: token
        };
        
        Ok(response)
    }
    else { 
        Err(AuthError::AuthError("auth failed".to_string(), StatusCode::FORBIDDEN))
    }
    
}


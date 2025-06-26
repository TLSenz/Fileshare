use axum::http::{StatusCode};
use axum::{ Json};
use axum::response::IntoResponse;
use crate::model::securitymodel::AuthError;
use crate::model::usermodel::{CreateUserRequest, LoginRequest, LoginResponse};
use crate::Security::jwt::encode_jwt;
use crate::service::userservice::{check_user_login, create_user};

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


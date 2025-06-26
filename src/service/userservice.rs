use crate::model::usermodel::{ConversionError, CreateUserRequest, LoginRequest, User};
use crate::repository::userrepository::{check_if_user_exist_login, create_user as other_create_user};

pub async fn create_user(user: CreateUserRequest) -> bool{
    
    let created_user = other_create_user(user).await;
    match created_user {
        Ok(_) => {
            true
        }
        Err(_) => {
            false
        }
    }
}
pub async fn check_user_login(user: LoginRequest) -> Result<bool, ConversionError>{
    if check_if_user_exist_login(user).await?{
        Ok(true)
    }
    else { 
        Ok(false)
    }
}




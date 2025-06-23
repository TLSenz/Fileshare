use crate::model::usermodel::{CreateUserRequest, FileToInsert};
use crate::repository::userrepository::{create_user as other_create_user};

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




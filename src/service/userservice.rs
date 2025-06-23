use crate::model::usermodel::ConversionError;
use std::fmt::{format, Error};
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::sync::atomic::compiler_fence;
use axum::extract::Multipart;
use bcrypt::{bcrypt, hash};
use serde::de::Unexpected::Str;
use crate::model::usermodel::{CreateUserRequest, FileToInsert};
use crate::repository::userrepository::{create_user as other_create_user, get_file_name_from_db, write_name_to_db};
use crate::schema::file_to_link::link;
use crate::model::usermodel::ConversionError::*;
use crate::model::filemodel::*;
use crate::schema::file::file_name;

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

pub async fn store_files(mut file: Multipart) -> Result<Vec<String>,ConversionError>{
    let mut links = Vec::new();

    while let Some(field) = file.next_field().await.unwrap() {
        let mut content_type = String::new();

        let other_file_name = field.name().unwrap().to_string();
        let file_type = field.content_type();

        match file_type {
            Some(file_type) => {
                let filetype_splited:Vec<&str> = file_type.split("/").collect();
                content_type = filetype_splited[1].to_string();
            }
            None => {
                content_type = "txt".to_string();
            }
        }

        let filename = "content/".to_owned() + other_file_name.as_str() + &"." + &content_type;
        let data = field.bytes().await.unwrap();
        println!("{}", filename);
        
        let size = data.len();
        let size = size.try_into()?;

        println!("Length of `{:?}` is {} bytes", other_file_name, data.len());
        let name_link_hash = hash(filename.clone(), 2)?;
        let data_hash = hash(data.clone(),2)?;
        
        
        let file_struct: FileToInsert = FileToInsert {
            file_name: other_file_name.clone(),
            hashed_file_name: name_link_hash.clone(), 
            content_hash: data_hash.clone(),        
            content_type: content_type.clone(),        
            size: size,                                 
            storage_path: filename.clone(),         
            owner_id: None,
            is_public: Some(1),                         
            is_deleted: Some(0),                        
        };



        let file = File::create(filename);
        match file {
            Ok(mut file) => {
                let file_write_result = file.write(&*data);

                match file_write_result {
                    Ok(..) => {
                        println!("Successfully wrote to File");
                    }
                    Err(..) => {
                        println!("Could not Write to File");
                    }
                }
            }
            Err(error) => {
                println!("Failing to write to File. Error is: {}", error );
            }
        }
        let other_link = create_link(file_struct).await?;
        links.push(other_link)
    }
    Ok(links)
}

pub async fn create_link(mut file:FileToInsert) -> Result<String,ConversionError>{

  let file = write_name_to_db(file).await;
    
    let mut files;
         match file {
        Ok(file) => {
            files = file
        }
        Err(error) => {
            return Err(ConversionError("error".to_string()))
        }
    };
    
    let other_link = format!("localhost:3000/api/{}", files.hashed_file_name);
    Ok(other_link)

}


pub async fn get_file_name(file_link: String) -> Result<GetFileResponse,Error> { // In Futur add checking for Same Name of File
    let file_link: Vec<_> = file_link.split("/").collect();
    let file_name_hash = file_link[file_link.len() - 1];
    
    let file = get_file_name_from_db(file_name_hash.to_string()).await?;
    
    let file_names = &file[0].file_name;
    let file_paths = &file[0].storage_path;
    
    let res:GetFileResponse = GetFileResponse{
        filename: file_names.to_string(),
        filepath: file_paths.to_string()
    };
    
    Ok(res)
}


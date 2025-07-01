use std::fmt::Error;
use std::fs;
use std::fs::{create_dir, File};
use std::io::Write;
use bytes::Bytes;
use aws_sdk_s3::primitives::ByteStream;
use axum::extract::Multipart;
use bcrypt::hash;
use crate::model::filemodel::GetFileResponse;
use crate::model::usermodel::{ConversionError, FileToInsert};
use crate::model::usermodel::ConversionError::*;
use crate::repository::filerepository::{check_if_file_name_exists, get_file_name_from_db, write_name_to_db};


pub async fn store_files(mut file: Multipart, username:&str) -> Result<Vec<String>,ConversionError>{
    let mut links = Vec::new();

    while let Some(field) = file.next_field().await? {
        let mut content_type = String::new();

        let other_file_name = field.name().unwrap().to_string();
        
        let check = check_if_file_name_exists(other_file_name.clone()).await?;
        
      
        
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
        println!("went to after contenttype");

        let filename = "content/".to_owned() + username.clone() + other_file_name.as_str() + &"." + &content_type;
        let data = field.bytes().await.map_err(ConversionError::from)?;

        println!("Went after Data");
        println!("{}", filename);

        let size = data.len();
        let size = size.try_into()?;

        println!("Length of `{:?}` is {} bytes", other_file_name, data.len());
        let name_link_hash = hash(filename.clone(), 4)?;
        let data_hash = hash(data.clone(),4)?;


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

        aws(&data, &file_struct).await?;
        write_data(&data, &file_struct).await?;

       
        let other_link = create_link(file_struct).await?;
        links.push(other_link)
    }
    Ok(links)
}

pub async fn create_link(file:FileToInsert) -> Result<String,ConversionError>{


    println!("File: {:?}", file);
    let file = write_name_to_db(file).await;

    let files;
    match file {
        Ok(file) => {
            files = file
        }
        Err(_) => {
            return Err(ConversionError("error".to_string()))
        }
    };
    println!("Filename: {}", &files.hashed_file_name);
    let other_link = format!("localhost:3000/api/download/{}", urlencoding::encode(files.hashed_file_name.as_str()));
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

pub async fn aws(data: &Bytes, data_info: &FileToInsert) -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let client = aws_sdk_s3::Client::new(&config);
    
    client.put_object()
        .bucket("fileshareapistorage")
        .key(&data_info.file_name)
        .body(ByteStream::from(data.to_vec()))
        .send()
        .await?;
        
    Ok(())
}

pub async fn write_data(data: &Bytes, data_info: &FileToInsert) -> Result<(), ConversionError>{

    let mut file = File::create(data_info.storage_path.clone()).map_err(|e| ConversionError::ConversionError("Error Creating File".to_string()))?;
    file.write(&*data).map_err(|e| ConversionError::ConversionError("Error writing Data to File".to_string()))?;
 
    Ok(())
}

pub fn create_folder_for_user(user_name:&str)->Result<(),ConversionError> {
    
    
    fs::create_dir(format!("/content/{}", user_name)).map_err(|e| ConversionError::ConversionError("Error Creating File".to_string()))?;
    Ok(())
}

pub fn get_folder_management(){
    
}

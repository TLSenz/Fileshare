use std::sync::Arc;
use axum::extract::{Multipart, Path};
use axum::body::*;
use axum::Extension;
use axum::response::IntoResponse;
use axum::http::{header, HeaderMap, Response, StatusCode};
use crate::model::securitymodel::EncodeJWT;
use crate::model::usermodel::ConversionError;
use crate::model::usermodel::ConversionError::*;
use crate::service::fileservice::{get_file_name, store_files};




pub async fn download(Path(file_link): Path<String> ,Extension(claims): Extension<Arc<EncodeJWT>>) -> impl IntoResponse{
    
    println!("Processing Request");

    let information = get_file_name(file_link).await;
     

    match information {
        Ok(infos) => {
            let content_types = mime_guess::from_path(&infos.filepath);
           let file_data = tokio::fs::read(&infos.filepath).await;
            match file_data {
                Ok(data) => {

                    let body = Body::from(data);
                    
                    Response::builder()
                        .header(header::CONTENT_TYPE, content_types.first_raw().unwrap())
                        .body(body)
                        .unwrap()

                }
                Err(_) => {
                    println!("Error Reading Data");
                     (StatusCode::INTERNAL_SERVER_ERROR, "File not found").into_response()
                }
            }

        }
        Err(error) => {
            println!("Error message while try to get File Path: {}", error);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }


}
#[axum::debug_handler]
pub async fn upload_file( Extension(claims): Extension<Arc<EncodeJWT>>, file: Multipart,) -> Result<String,ConversionError>{

    let is_stored = store_files(file, &claims.username).await;
    match is_stored {
        Ok(links) => {

            if let Some(first_link) = links.into_iter().next() {
                Ok(first_link)
            } else {
                println!("Hello World second  Erorr");
                Err(ConversionError("error".to_string()))
            }
        }
        Err(error) => {
            println!("Hello World first Erorr");
            Err(error)
        }
    }
    
}

pub async fn folder_management(Path(folder_action): Path<char>)-> impl IntoResponse{
    
    match folder_action { 
        m=> {
            
        }
        d => {
            
        }
        e => {
                
        }
        g => {
            
        }
        
    }
}



use axum::extract::Path;
use axum::body::*;
use axum::response::IntoResponse;
use axum::http::{header, Response, StatusCode};
use crate::service::userservice::get_file_name;



pub async fn download(Path(file_link): Path<String>) -> impl IntoResponse{
    
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
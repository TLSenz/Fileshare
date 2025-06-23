
use axum::extract::Path;
use axum::body::*;
use axum::response::IntoResponse;
use axum::http::{header, HeaderMap, StatusCode};
use jsonwebtoken::Header;
use crate::service::userservice::get_file_name;

pub async fn download(Path(file_link): Path<String>) -> impl IntoResponse{

    let information = get_file_name(file_link).await;



    match information {
        Ok(infos) => {
            let content_types = mime_guess::from_path(&infos.filepath);
           let file_data = tokio::fs::read(&infos.filepath).await;

            match file_data {
                Ok(data) => {

                    let mut headers = HeaderMap::new();
                    headers.insert(header::CONTENT_TYPE, content_types.first_raw().unwrap().parse().unwrap());
                    let content_disposition_value = format!("attachment; filename=\"{}\"", infos.filename);
                    headers.insert(header::CONTENT_DISPOSITION, content_disposition_value.parse().unwrap())

                    let stream = BodyDataStream.;

                    // 5. Wrap the stream in `StreamBody`.
                    // `StreamBody` is an `axum::body::HttpBody` implementation that takes a stream of data.
                    let body = StreamBody::new(stream);
                }
                Err(error) => {
                    println!("Error Reading Data")
                }
            }

        }
        Err(error) => {
            println!("Error message while try to get File Path: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }


}
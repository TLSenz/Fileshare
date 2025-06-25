use crate::Security::JWT::AuthError::AuthError;
use std::env;
use std::fmt::Error;
use axum::{http, Error};
use axum::http::{HeaderValue, Response, StatusCode};
use axum::extract::Request;
use axum::middleware::Next;
use dotenv::dotenv;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header, Validation, decode, DecodingKey, TokenData};
use crate::model::securitymodel::{AuthError, EncodeJWT};
use crate::model::securitymodel::AuthError::AuthError;
use crate::model::usermodel::ConversionError;

pub fn encode_jwt(name: &str, email: &str) -> Result<String, ConversionError>{

    let jwt_info = EncodeJWT {
        username: name.to_string(),
        email: email.to_string()
    };

    dotenv().ok();
    let secret = env::var("JWT_SECRET")?;
    let token = encode(&Header::default(), &jwt_info, &EncodingKey::from_secret(secret.as_ref())).unwrap();
    Ok(token)
}

pub fn decode_jwt(jwt_token: String)->  Result<TokenData<EncodeJWT>, ConversionError>{

    dotenv().ok();
    let secret = env::var("JWT_SECRET")?;
    let token_message = decode::<EncodeJWT>(&jwt_token, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256)).unwrap();
    Ok(token_message)
}

pub async fn authenticate(mut req:Request, next: Next ) -> Result<Response<Header>, AuthError>{
    let auth_header = req.headers().get(http::header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => { header.to_str().map_err(|_| AuthError("Empty header is not allowed".to_string(), StatusCode::FORBIDDEN))},
        None => { Err(AuthError("Please add JWT to your Header".to_string(), StatusCode::FORBIDDEN)) }
    }?;
    
    let mut header = auth_header.split_whitespace();
    let (bearer, token) = (header.next(), header.next());
    let token_data = decode_jwt(token.unwrap().to_string());
    
    todo!()
}
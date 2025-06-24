use std::env;
use dotenv::dotenv;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header, Validation, decode, DecodingKey, TokenData};
use crate::model::securitymodel::EncodeJWT;
use crate::model::usermodel::ConversionError;

pub async fn encode_jwt(name: &str, email: &str) -> Result<String, ConversionError>{

    let jwt_info = EncodeJWT {
        username: name.to_string(),
        email: email.to_string()
    };

    dotenv().ok();
    let secret = env::var("JWT_SECRET")?;
    let token = encode(&Header::default(), &jwt_info, &EncodingKey::from_secret(secret.as_ref())).unwrap();
    Ok(token)
}

pub async fn decode_jwt(jwt_token: String)->  Result<TokenData<EncodeJWT>, ConversionError>{

    dotenv().ok();
    let secret = env::var("JWT_SECRET")?;
    let token_message = decode::<EncodeJWT>(&jwt_token, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256)).unwrap();
    Ok(token_message)
}
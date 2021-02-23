use crate::errors::ServiceError;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::error::Error;


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub fn validate_token(token: &str) -> Result<bool, ServiceError> {

println!("{:?}", token);
    let token_decode = decode::<Claims>(&token, &DecodingKey::from_secret("mon_secret".as_ref()), &Validation::default());
    println!("{:?}",token_decode);
    Ok(token_decode.is_ok())
}


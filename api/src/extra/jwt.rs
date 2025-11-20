use chrono::{Duration, Utc};
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode,
    errors::{ ErrorKind},Algorithm
};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize,Clone)]
pub struct Claims {
    pub sub: String,
    exp: usize,
}

impl Claims {
    pub fn create_token(user_id: String) -> Result<String, ErrorKind> {
        dotenvy::dotenv().ok();
        let claims = Claims {
            sub: user_id,
            exp: (Utc::now() + Duration::days(2)).timestamp() as usize,
        };
        let secret = std::env::var("JWT_SECRET").map_err(|_| ErrorKind::InvalidKeyFormat)?;
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|_| ErrorKind::InvalidKeyFormat)?;

        Ok(token)
    }

    pub fn decode_token(token: String) -> Result<Claims, ErrorKind> {
        dotenvy::dotenv().ok();
        let secret = std::env::var("JWT_SECRET").unwrap();
        let validate = Validation {
            validate_exp: true,
            algorithms: vec![Algorithm::HS256],
            ..Validation::default()
        };
        let decoded = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validate,
        );

        match decoded {
            Ok(data) => Ok(data.claims),
            Err(_) => Err(ErrorKind::InvalidToken),
        }
    }
}

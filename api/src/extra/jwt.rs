
use jsonwebtoken :: {DecodingKey, decode};
use poem::{Error, http::StatusCode};
use serde::{Serialize, Deserialize};
#[derive(Serialize,Deserialize)]
pub struct Claims{
    pub sub:String,
    exp:usize
}

impl Claims{
    pub fn create_token(user_id:String)->String{
        dotenvy::dotenv().ok();
        let claims = Claims {
            sub: user_id,
            exp,
        };
        let secret = std::env::var("JWT_SECRET").unwrap();
        match secret{
            Ok(secret)=>{
                let token = encode(&Header::default(),&claims,&EncdeingKey::from_secret(secret.as_ref()));
                return token;
            }
            Err(_)=>{
                return Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR));
            }
        }
    }

    pub fn decode_token(token:String)->Result<Claims,Error>{
        dotenvy::dotenv().ok();
        let secret = std::env::var("JWT_SECRET").unwrap();
        let decoded = decode::<Claims>(&token,&DecodingKey::from_secret(secret.as_ref()),validation::default());
        match decoded{
            Ok(data)=>{
                Ok((data.claims))
            }
            Err(_)=>{
                Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))
            }
        }
    }
}
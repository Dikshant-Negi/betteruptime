use poem::{Error, FromRequest, Request, http::StatusCode};
pub struct UserId(pub String);
use crate::extra::jwt::Claims;

impl<'a> FromRequest<'a> for UserId{
    async fn from_request(
        req: &'a Request,
        body: &mut RequestBody,
    ) -> Request<Self>{
        let header = req.headers().get("authorization").and_then(|value|value.to_str().ok()).ok_or_else(err|| Error::from_string("missing token", StatusCode::UNAUTHORIZED))?;
        
        let user_id = Claims::decode_token(&header);

        match user_id{
            Ok(claims)=>{
                Ok(claims.sub)
            }
            Err(_)=>{
                Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))
            }
        }

    }
}
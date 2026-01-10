use poem::{Error, Request, handler, http::StatusCode, web::{Data, Json}};
use crate::extra::website_response::{WebsiteInput,WebsiteOutput};
use crate::extra::app_state::AppState;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::extra::auth_middleware::Token;
use redis_stream::redis_client::RedisStream;

#[handler]
pub async fn create_website(Json(body):Json<WebsiteInput>,data:Data<&Arc<Mutex<AppState>>>,req:&Request)->Result<Json<WebsiteOutput>,Error>{
    if body.url.is_empty() || body.name.is_empty() || body.interval <=0{
        return Err(Error::from_status(StatusCode::BAD_REQUEST));
    }
    let user = req.extensions().get::<Token>().ok_or_else(|| Error::from_status(StatusCode::UNAUTHORIZED));
    let id = match user {
        Ok(t) => t,
        Err(e) => return Err(e),
    };
    let mut lock = data.lock().await;
    let mut db = lock.db.lock().await;
    let res = db.create_websites(body.url,id.user_id.clone(),body.name,body.interval).await;

    match res{
        Ok(_)=>{
            RedisStream::schedule_website(&mut self, id, website_url, interval_sec);
            Ok(Json(WebsiteOutput { success: (true), message: String::from("website inserted successfully") }))

        }
        Err(_)=>{
            return Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR));
        }
    }
}


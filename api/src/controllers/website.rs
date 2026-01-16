use poem::{Error, Request, handler, http::StatusCode, web::{Data, Json}};
use crate::extra::website_response::{WebsiteInput,WebsiteOutput};
use crate::extra::app_state::AppState;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::extra::auth_middleware::Token;
use redis_stream::redis_client::RedisStream;

#[handler]
pub async fn create_website(Json(body):Json<WebsiteInput>,data:Data<&Arc<Mutex<AppState>>>,req:&Request)->Result<Json<WebsiteOutput>,Error>{
    // 1. Validation (Same)
    if body.url.is_empty() || body.name.is_empty() || body.interval <=0{
        return Err(Error::from_status(StatusCode::BAD_REQUEST));
    }

    // 2. Auth Token extraction (Same)
    let user = req.extensions().get::<Token>().ok_or_else(|| Error::from_status(StatusCode::UNAUTHORIZED));
    let id = match user {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    let mut lock = data.lock().await;
    let mut db = lock.db.lock().await;
    
    // NEW STEP 1: Database se User ka Email nikalo
    // Note: Ye tabhi chalega agar tumne pichle step mein DB file mein 'get_user_email' function add kiya hai.
    let user_email = match db.get_user_email(&id.user_id).await {
        Ok(email) => email,
        Err(_) => return Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR)),
    };

    let mut redis_client = lock.redis.lock().await;
    
    // 3. Create Website in DB (Same)
    let res = db.create_websites(body.url,id.user_id.clone(),body.name,body.interval).await;

    match res{
        Ok(r)=>{
            // NEW STEP 2: Schedule karte waqt Email pass karo
            // Note: Ye abhi RED LINE (Error) dikhayega jab tak hum agli file (redis_client.rs) update nahi karte.
            let _ = RedisStream::schedule_website(
                &mut redis_client, 
                &r.id, 
                r.url.as_str(), 
                r.check_interval.unwrap_or(60), 
                &user_email // <--- Email Add Kiya
            );
            
            Ok(Json(WebsiteOutput { success: (true), message: String::from("website inserted successfully") }))
        }
        Err(_)=>{
            return Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR));
        }
    }
}
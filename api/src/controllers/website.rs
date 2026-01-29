use poem::{Error, Request, handler, http::StatusCode, web::{Data, Json, Path}};
use crate::extra::website_response::{WebsiteInput,WebsiteOutput};
use crate::extra::app_state::AppState;
use crate::extra::auth_middleware::Token;
use std::sync::Arc;
use tokio::sync::Mutex;
use redis_stream::redis_client::RedisStream;
use store::models::website::{ Website, DailyReliabilityStat};

#[handler]
pub async fn create_website(Json(body):Json<WebsiteInput>,data:Data<&Arc<Mutex<AppState>>>,req:&Request)->Result<Json<WebsiteOutput>,Error>{
   
    if body.url.is_empty() || body.name.is_empty() || body.check_interval <=0{
        return Err(Error::from_status(StatusCode::BAD_REQUEST));
    }

    let token = req.extensions().get::<Token>().ok_or_else(|| Error::from_status(StatusCode::UNAUTHORIZED));
    let id = match token {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    let mut lock = data.lock().await;
    let mut db = lock.db.lock().await;
    

    let mut redis_client = lock.redis.lock().await;
    
    let res = db.create_websites(body.url,id.user_id.clone(),body.name,body.check_interval).await;

    match res{
        Ok(r)=>{
            let _ = RedisStream::schedule_website(
                &mut redis_client, 
                &r.id, 
                r.url.as_str(), 
                r.check_interval.unwrap_or(60), 
                &id.user_id
            );
            
            Ok(Json(WebsiteOutput { success: (true), message: String::from("website inserted successfully") }))
        }
        Err(_)=>{
            return Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR));
        }
    }
}
#[handler]
pub async fn get_websites(
    req: &Request,
    data: Data<&Arc<Mutex<AppState>>>,
) -> poem::Result<Json<Vec<Website>>> {
    
    let token = req.extensions().get::<Token>().ok_or_else(|| {
        poem::Error::from_string("Unauthorized - Token missing", StatusCode::UNAUTHORIZED)
    })?;

    let state = data.lock().await;       
    let store = state.db.lock().await; 

    let websites = store.get_websites(&token.user_id) 
        .await
        .map_err(|e| poem::Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(websites))
}
#[handler]
pub async fn get_reliability_graph(Path(id): Path<String>, data: Data<&Arc<Mutex<AppState>>>) -> poem::Result<Json<Vec<DailyReliabilityStat>>> {
    let state = data.lock().await;
    let store = state.db.lock().await;
    let stats = store.get_daily_reliability(&id)
        .await
        .map_err(|e| poem::Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(stats))
}
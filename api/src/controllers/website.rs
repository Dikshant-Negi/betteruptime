use store::Store;
use poem::{Error, handler, http::StatusCode, web::{Data, Json}};
use crate::extra::website_response::{WebsiteInput,WebsiteOutput};
use std::sync::Arc;
use tokio::sync::Mutex;
#[handler]
pub async fn create_website(Json(body):Json<WebsiteInput>,data:Data<&Arc<Mutex<Store>>>)->Result<Json<WebsiteOutput>,Error>{
    if body.url.is_empty() || body.name.is_empty() || body.interval <=0 {
        return Err(Error::from_status(StatusCode::BAD_REQUEST));
    }

    let mut lock = data.lock().await;
    let res = lock.create_website(body.url,String::from("123456"),body.name,body.interval).await;

    match res{
        Ok(_)=>{
            Ok(Json(WebsiteOutput { success: (true), message: String::from("website inserted successfully") }))
        }
        Err(_)=>{
            return Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR));
        }
    }
}

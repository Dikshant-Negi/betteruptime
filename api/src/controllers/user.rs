use poem::{handler, http::StatusCode, web::{Data, Json},Error};
use store::Store;

use std::{ sync::Arc};
use tokio::sync::Mutex;
use jwt::Claims;
use crate::extra::{jwt::Claims, user_response,auth_middleware::UserId};


//add jwt auth
#[handler]
pub async fn  create_user(Json(body):Json<user_response::CreateUserInput>,data:Data<&Arc<Mutex<Store>>>)->Result<Json<user_response::CreateUserOuptput>,Error>{
    if body.username.is_empty() || body.email.is_empty() || body.password.is_empty(){
        return Err(Error::from_status(StatusCode::BAD_REQUEST));
    }

    let token = Claims::create_token(user_id);  
    let mut lock = data.lock().await;


    let res = lock.create_user(body.email,body.password,body.username).await;

    match res{
        Ok(_)=>{
           Ok( Json(user_response::CreateUserOuptput { success: (true), jwt: token , message:  String::from("User created successfully") }))
        }
        Err(_)=>{
           Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

#[handler]
pub async fn sigin(Json(body):Json<user_response::SignInput>,data:Data<&Arc<Mutex<Store>>>,UserId(user_id):UserId)->Result<Json<user_response::CreateUserOuptput>,Error>{
     if  body.email.is_empty() || body.password.is_empty(){
        return Err(Error::from_status(StatusCode::BAD_REQUEST))
    }
    let mut lock = data.lock().await;
    let res  = lock.sigin(body.email,body.password).await;

    match res{
        Ok(_)=>{
           Ok( Json(user_response::CreateUserOuptput{
                success:true,
                jwt:String::from("123456"),
                message:String::from("user signed in successfully")
            }))
        }
        Err(_)=>{
            Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}




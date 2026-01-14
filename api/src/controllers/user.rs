use crate::extra::app_state::AppState;
use crate::extra::jwt::Claims;
use crate::extra::user_response;
use poem::{
    Error, handler,
    http::StatusCode,
    web::{Data, Json},
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

//add jwt auth
#[handler]
pub async fn create_user(
    Json(body): Json<user_response::CreateUserInput>,
    data: Data<&Arc<Mutex<AppState>>>,
) -> Result<Json<user_response::CreateUserOuptput>, Error> {
    print!("creating user:{}", body.email);
    if body.username.is_empty() || body.email.is_empty() || body.password.is_empty() {
        return Err(Error::from_status(StatusCode::BAD_REQUEST));
    }
    print!("creating user:{}", body.email);
    let state = data.lock().await;
    let mut db = state.db.lock().await;
    let res = db
        .create_user(body.email, body.password, body.username)
        .await;

    match res {
        Ok(id) => {
            let token = Claims::create_token(id);
            match token {
                Ok(t) => {
                    return Ok(Json(user_response::CreateUserOuptput {
                        success: (true),
                        jwt: t,
                        message: String::from("User created successfully"),
                    }));
                }
                Err(e) => {
                 
                    let err = json!({
                        "success": false,
                        "error": "JWT creation failed"
                    });
                    eprintln!("JWT creation error: {:?}", e);
                    Err(Error::from_string(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR))
                }
            }
        }
        Err(e) => {
            if let Some(db_err) = e.as_database_error() {
                let msg = db_err.message();

                if msg.contains("users_email_key") {
                    return Err(Error::from_status(StatusCode::CONFLICT)); 
                }
            }

            eprintln!("DB error: {:?}", e);
            Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

#[handler]
pub async fn sigin(
    Json(body): Json<user_response::SignInput>,
    data: Data<&Arc<Mutex<AppState>>>,
) -> Result<Json<user_response::CreateUserOuptput>, Error> {
    if body.email.is_empty() || body.password.is_empty() {
        return Err(Error::from_status(StatusCode::BAD_REQUEST));
    }
    let mut lock = data.lock().await;
    let mut db = lock.db.lock().await;
    let res = db.sigin(body.email, body.password).await;

    match res {
        Ok(id) => {
            let token = Claims::create_token(id);
            match token {
                Ok(t) => {
                    return Ok(Json(user_response::CreateUserOuptput {
                        success: (true),
                        jwt: t,
                        message: String::from("User signed in successfully"),
                    }));
                }
                Err(_) => {
                    return Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR));
                }
            }
        }
        Err(_) => Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

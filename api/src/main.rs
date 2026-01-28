use crate::controllers::{user, website};
use crate::extra::app_state::AppState;
use crate::extra::auth_middleware::TokenMiddleware;
use poem::{EndpointExt, Route, Server, listener::TcpListener, post, middleware::Cors};
use redis_stream::redis_client::RedisStream;
use std::sync::Arc;
use store::Store;
use tokio::sync::Mutex;
pub mod controllers;
pub mod extra;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();
    let mut db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let mut redis_url = std::env::var("REDIS_URL").expect("REDIS_URL");
    print!("redis url:{}", redis_url);
    let db = match Store::new(&mut db_url).await {
        Ok(store) => Arc::new(Mutex::new(store)),
        Err(e) => {
            panic!("Failed to connect to database: {}", e);
        }
    };
    let redis = match RedisStream::new(&mut redis_url) {
        Ok(mut client) => {
            let consumer = client.add_consumer();
            match consumer {
                Ok(_) => {
                    println!("Redis consumer added successfully.");
                }
                Err(e) => {
                    panic!("Failed to add redis consumer: {}", e);
                }
            }
            Arc::new(Mutex::new(client))
        }
        Err(e) => {
            panic!("Failed to connect to redis: {}", e);
        }
    };
    let state = Arc::new(Mutex::new(AppState { db, redis }));

    let api = Route::new()
        .at("/createuser", post(user::create_user))
        .at("/signin", post(user::sigin))
        .at(
            "/createwebsite",
            post(website::create_website).with(TokenMiddleware),
        )
        .at(
            "/getwebsites",
            poem::get(website::get_websites).with(TokenMiddleware)
        );
    
    let cors = Cors::new()
        .allow_origin("http://localhost:5173")
        .allow_methods(vec![poem::http::Method::GET, poem::http::Method::POST])
        .allow_credentials(true);

    let app = Route::new().nest("/api", api).data(state.clone()).with(cors);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("betteruptime")
        .run(app)
        .await
}

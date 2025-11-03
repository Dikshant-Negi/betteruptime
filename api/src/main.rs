
use poem::{listener::TcpListener, post, Route, Server,EndpointExt};
use store::Store;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::controllers::{user,website};

pub mod extra;
pub mod controllers;




#[tokio::main(flavor = "multi_thread")]
async fn main()->Result<(),std::io::Error> {
    dotenvy::dotenv().ok();
    let mut db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let conn = Store::new(&mut db_url).await;
    let conn = match conn{
        Ok(c)=>{
            c
        }
        Err(e)=>{
            panic!("DB Connection Error: {}",e);
        }
    };
    let arc_conn = Arc::new(Mutex::new(conn));

    let app = Route::new()
    .at("/createuser",post(user::create_user))
    .at("/signin",post(user::sigin))
    .at("/createwebsite",post(website::create_website))
    
    .data(arc_conn.clone());

    Server::new(TcpListener::bind("0.0.0.0:3000")).name("betteruptime").run(app).await
}

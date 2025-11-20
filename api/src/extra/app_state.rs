use std::{ sync::Arc};
use tokio::sync::Mutex;
use store::Store;
use redis_stream::redis_client::RedisStream;
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Store>>,
    pub redis: Arc<Mutex<RedisStream>>,
} 

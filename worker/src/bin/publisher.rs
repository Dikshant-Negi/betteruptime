
use redis_stream::redis_client::RedisStream;
use tokio;

pub fn insert_into_stream(){
    dotenvy::dotenv().ok();
    let  redis_url = std::env::var("REDIS_URL").expect("REDIS_URL");
    print!("redis url: {}", redis_url);
    match RedisStream::new(&redis_url) {
        Ok(mut client) => {
            println!("Connected to Redis stream.");
            loop {
                match client.process_due_websites() {
                    Ok(_) => {
                        std::thread::sleep(std::time::Duration::from_millis(500));
                    }
                    Err(e) => {
                        eprintln!("Error processing due websites: {}", e);
                        break;
                    }
                }
            }

            println!("Connection lost, reconnecting...");
            std::thread::sleep(std::time::Duration::from_secs(1));
          
        }
        Err(e) => {
            eprintln!("Failed to connect to Redis: {}", e);
            std::thread::sleep(std::time::Duration::from_secs(1));
          
        }
    }
}

#[tokio::main]
async fn main() {
    insert_into_stream();
}

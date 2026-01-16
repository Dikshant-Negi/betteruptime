use futures::stream::StreamExt;
use futures::stream::FuturesUnordered;
use redis::{self, Commands, RedisResult};
use redis_stream::redis_client::RedisStream;
use store::Store;
use tokio::sync::Semaphore;
use uuid::Uuid;
use std::sync::Arc;


pub struct PingResult {
    pub job_id: String,
    pub website_id: String,
    pub url: String,
    pub user_id: String,
    pub response: Result<reqwest::Response, reqwest::Error>,
}
// 1. Logic to push alerts to Redis for the Notifier
fn send_alert_to_notifier(
    client: &mut RedisStream,
    website_url: &str,
    reason: &str,
    user_email: &str,
) -> RedisResult<()> {
    println!("Pushing alert to stream for email: {}", user_email);
    let _: () = client.conn.xadd(
        "betteruptime:alerts",
        "*",
        &[
            ("website_url", website_url),
            ("user_email", user_email),
            ("reason", reason),
        ],
    )?;
    Ok(())
}

// 2. Simple HTTP Checker
async fn handle_website_check(
    url: String,
    http: Arc<reqwest::Client>,
) -> Result<reqwest::Response, reqwest::Error> {
    http.get(url).send().await
}

// 3. The Failure Manager: Runs in background
async fn process_failure(
    mut client: RedisStream, 
    store: Arc<Store>,      
    user_id: String,        
    url: String,
    reason: String,
    last_status: String,
    status_key: String,
) {
    if last_status == "UP" {
        println!("State Change detected (UP -> DOWN) for {}", url);

        // Fetch user email from DB
        match store.get_user_email(&user_id).await {
            Ok(email) => {
                // Set status to DOWN in Redis
                let _: () = client.conn.set(&status_key, "DOWN").unwrap();
                // Send alert to notifier stream
                let _ = send_alert_to_notifier(&mut client, &url, &reason, &email);
            }
            Err(e) => eprintln!("Failed to fetch email for user {}: {}", user_id, e),
        }
    }
}

pub async fn process_jobs() -> RedisResult<()> {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Ensure Store::new accepts &String in store/src/store.rs
    let store_instance = Store::new(&db_url).await.expect("Failed to connect to DB");
    let store = Arc::new(store_instance);

    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let mut client = RedisStream::new(&redis_url)?;

    client.add_consumer()?; 
    let consumer_name = format!("worker-{}", Uuid::new_v4());
    let http = Arc::new(reqwest::Client::new());
    let semaphore = Arc::new(Semaphore::new(500)); 
    let mut futures = FuturesUnordered::new();

    println!("Worker running as: {}", consumer_name);

    loop {
        // Step A: Read jobs from group
        if let Ok(reply) = client.x_read_group(&consumer_name) {
            for stream in reply.keys {
                for job in stream.ids {
                    let website_id = job.map.get("website_id").map(|v| String::from_utf8_lossy(match v { redis::Value::Data(d) => d, _ => &[] }).to_string()).unwrap_or_default();
                    let url = job.map.get("url").map(|v| String::from_utf8_lossy(match v { redis::Value::Data(d) => d, _ => &[] }).to_string()).unwrap_or_default();
                    let user_id = job.map.get("user_id").map(|v| String::from_utf8_lossy(match v { redis::Value::Data(d) => d, _ => &[] }).to_string()).unwrap_or_default();

                    if website_id.is_empty() || url.is_empty() { continue; }

                    let job_id = job.id.clone();
                    let http_clone = http.clone();
                    let permit = semaphore.clone().acquire_owned().await.unwrap();
                    
                    let u_id = user_id.clone();
                    let w_id = website_id.clone();
                    let u_url = url.clone();

                    futures.push(async move {
                        let _permit = permit; 
                        let res = handle_website_check(u_url.clone(), http_clone).await;

                        PingResult {
                            job_id,
                            website_id: w_id,
                            url: u_url,
                            user_id: u_id,
                            response: res,
                        }
                    });
                }
            }
        }

        // Step B: Process Results
        while let Some(ping) = futures.next().await {
            let status_key = format!("betteruptime:status:{}", ping.website_id);
            
            match ping.response {
                Ok(_) => {
                    let last_status: String = client.conn.get(&status_key).unwrap_or("UP".to_string());
                    if last_status == "DOWN" {
                        println!("RECOVERY: {} is UP again", ping.url);
                        let _: () = client.conn.set(&status_key, "UP").unwrap();
                    }
                    let _:() = client.conn.xack("betteruptime:website", "uptime-checkers", &[&ping.job_id]).unwrap();
                }
                Err(e) => {
                    let last_status: String = client.conn.get(&status_key).unwrap_or("UP".to_string());
                    
                    // CLONE DATA (but not the client)
                    let store_clone = store.clone();
                    let err_msg = e.to_string();
                    let u_id = ping.user_id.clone();
                    let u_url = ping.url.clone();
                    let s_key = status_key.clone();
                    let r_url = redis_url.clone(); 

                    // Spawn background task with its own Redis connection
                    tokio::spawn(async move {
                        if let Ok(local_client) = RedisStream::new(&r_url) {
                            process_failure(
                                local_client,
                                store_clone,
                                u_id,
                                u_url,
                                err_msg,
                                last_status,
                                s_key,
                            ).await;
                        }
                    });

                    let _:() = client.conn.xack("betteruptime:website", "uptime-checkers", &[&ping.job_id]).unwrap();
                }
            }
        }
    }
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    if let Err(e) = rt.block_on(process_jobs()) {
        eprintln!("Fatal Error: {}", e);
    }
}
use worker::analytics;
use futures::stream::StreamExt;
use futures::stream::FuturesUnordered;
use redis::{self, Commands, RedisResult};
use redis_stream::redis_client::RedisStream;
use store::Store;
use tokio::sync::Semaphore;
use uuid::Uuid;
use std::sync::Arc;
use std::time::Instant; 

pub struct PingResult {
    pub job_id: String,
    pub website_id: String,
    pub url: String,
    pub user_id: String,
    pub is_up: bool,      
    pub latency_ms: u64,    
    pub error_msg: Option<String>,
}

// Logic to push alerts to Redis for the Notifier
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

// Updated HTTP Checker (Calculates Latency)
async fn handle_website_check(
    url: String,
    http: Arc<reqwest::Client>,
) -> (bool, u64, Option<String>) {
    let start = Instant::now();
    // 10s timeout to prevent hanging
    let resp = http.get(&url).timeout(std::time::Duration::from_secs(10)).send().await;
    let latency = start.elapsed().as_millis() as u64;

    match resp {
        Ok(res) => {
            let is_up = res.status().is_success();
            let error = if !is_up { Some(format!("Status: {}", res.status())) } else { None };
            (is_up, latency, error)
        }
        Err(e) => (false, latency, Some(e.to_string())),
    }
}

pub async fn process_jobs() -> RedisResult<()> {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
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
        //Read jobs from group
        match client.x_read_group(&consumer_name) {
            Ok(reply) => {
                for stream in reply.keys {
                    for job in stream.ids {
                        let website_id = job.map.get("website_id")
                            .or_else(|| job.map.get("id"))
                            .map(|v| String::from_utf8_lossy(match v { redis::Value::Data(d) => d, _ => &[] }).to_string())
                            .unwrap_or_default();

                        let url = job.map.get("url")
                            .map(|v| String::from_utf8_lossy(match v { redis::Value::Data(d) => d, _ => &[] }).to_string())
                            .unwrap_or_default();

                        let user_id = job.map.get("user_id")
                            .map(|v| String::from_utf8_lossy(match v { redis::Value::Data(d) => d, _ => &[] }).to_string())
                            .unwrap_or_default();

                        if website_id.is_empty() || url.is_empty() { 
                            println!("Invalid Job Found (Empty ID/URL). Acking and skipping.");
                            let _:() = client.conn.xack("betteruptime:website", "uptime-checkers", &[&job.id]).unwrap();
                            continue; 
                        }

                        let job_id = job.id.clone();
                        let http_clone = http.clone();
                        let permit = semaphore.clone().acquire_owned().await.unwrap();
                        
                        let u_id = user_id.clone();
                        let w_id = website_id.clone();
                        let u_url = url.clone();

                        futures.push(async move {
                            let _permit = permit;
                            let (is_up, latency, error_msg) = handle_website_check(u_url.clone(), http_clone).await;

                            PingResult {
                                job_id,
                                website_id: w_id,
                                url: u_url,
                                user_id: u_id,
                                is_up,
                                latency_ms: latency,
                                error_msg,
                            }
                        });
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                continue;
            }
        }

        //Process Results
        while let Some(ping) = futures.next().await {

            let status_key = format!("betteruptime::status:{}", ping.website_id);
            let last_redis_status: String = client.conn.get(&status_key).unwrap_or("UP".to_string());

            
            let analytics_data = analytics::CheckResult {
                website_id: ping.website_id.clone(),
                is_up: ping.is_up,
                response_time_ms: ping.latency_ms,
                error_msg: ping.error_msg.clone(),
                previous_status: last_redis_status.clone(),
            };
            
            if let Err(e) = analytics::process_check_result(&store, analytics_data).await {
                eprintln!("Analytics DB Error for {}: {}", ping.website_id, e);
            }

            //CHECK STATUS & ALERT
            if ping.is_up {
                // Currently UP
                println!("Checked {}: UP ({}ms)", ping.url, ping.latency_ms);
                if last_redis_status == "DOWN" {
                    println!("RECOVERY: {} is UP again", ping.url);
                    let _: () = client.conn.set(&status_key, "UP").unwrap();
                }
            } else {
                // Currently DOWN
                if last_redis_status == "UP" {
                    println!("CRASH: {} went DOWN. Reason: {:?}", ping.url, ping.error_msg);
                    let _: () = client.conn.set(&status_key, "DOWN").unwrap();

                    // Send Alert
                    if let Ok(email) = store.get_user_email(&ping.user_id).await {
                        let reason = ping.error_msg.unwrap_or("Unknown Error".to_string());
                        let _ = send_alert_to_notifier(&mut client, &ping.url, &reason, &email);
                    }
                }
            }

            // 3. ACK JOB
            let _:() = client.conn.xack("betteruptime:website", "uptime-checkers", &[&ping.job_id]).unwrap();
        }
    }
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    if let Err(e) = rt.block_on(process_jobs()) {
        eprintln!("Fatal Error: {}", e);
    }
}
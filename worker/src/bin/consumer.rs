use futures::{StreamExt, stream::FuturesUnordered};
use redis_stream::redis_client::RedisStream;
use reqwest;
use std::sync::Arc;
use tokio;
use tokio::sync::Semaphore;
use uuid::Uuid;
pub async fn consume_from_stream() {
    dotenvy::dotenv().ok();
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL");
    let id = Uuid::new_v4();
    let consumer_name = format!("consumer-{}", id);
    let semaphore = Arc::new(Semaphore::new(50));

    match RedisStream::new(&redis_url) {
        Ok(mut client) => {
            loop {
                let jobs = match client.x_read_group(&consumer_name) {
                    Ok(j) => j,
                    Err(e) => {
                        eprintln!("Redis read error: {}", e);
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        continue;
                    }
                };

                let mut futures = FuturesUnordered::new();

                for entry in jobs.keys {
                    for job in entry.ids {
                        let p = semaphore.clone().acquire_owned().await;
                        let permit = match p {
                            Ok(_p) => _p,
                            Err(e) => {
                                eprintln!("Semaphore acquire error: {}", e);
                                continue;
                            }
                        };
                        futures.push( async move {
                            let website_url = match job.map.get("website_url") {
                                Some(redis::Value::Data(bytes)) => {
                                    std::str::from_utf8(bytes).unwrap()
                                }
                                _ => return,
                            };

                            match reqwest::get(website_url).await {
                                Ok(resp) => println!("{} → {}", website_url, resp.status()),
                                Err(e) => eprintln!("{} failed: {}", website_url, e),
                            }

                            drop(permit);
                            // Here later you will ACK
                            // client.x_ack(job.id)
                        });
                        while let Some(result) = futures.next().await {
                                println!("{:?}", result);
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to Redis: {}", e);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}

#[tokio::main]
async fn main() {
    consume_from_stream().await;
}

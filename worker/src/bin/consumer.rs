use futures::stream::FuturesUnordered;
use redis::RedisResult;
use redis_stream::redis_client::RedisStream;
use reqwest;
use tokio;
use uuid::Uuid;

pub async fn consume_from_stream() -> RedisResult<()> {
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL");
    let id = Uuid::new_v4();
    let consumer_name = format!("consumer-{}", id);
    match RedisStream::new(&redis_url) {
        Ok(mut client) => {
            match client.x_read_group(consumer_name.as_str()) {
                Ok(jobs) => {
                    let  futures = FuturesUnordered::new();
                    for entry in jobs.keys {
                        for job in entry.ids {
                            futures.push(tokio::spawn(async move {
                                //println!(job.map.get("website_url").unwrap());
                                let website_value = job.map.get("website_url");
                                let website_url = match website_value {
                                    Some(redis::Value::Data(bytes)) => {
                                        match std::str::from_utf8(bytes) {
                                            Ok(s) => s,
                                            Err(_) => {
                                                eprintln!("website_url is not valid UTF-8");
                                                return Err(());
                                            }
                                        }
                                    }
                                    Some(other) => {
                                        eprintln!("website_url is not Data type: {:?}", other);
                                        return Err(());
                                    }
                                    None => {
                                        eprintln!("No website_url found in the job.");
                                        return Err(());
                                    }
                                };
                                let response = reqwest::get(&website_url.to_string()).await;
                                match response {
                                    Ok(resp) => {
                                        let status = resp.status();
                                        println!("{:?} → {} ", website_url, status);
                                        Ok(())
                                    }
                                    Err(e) => {
                                        eprintln!("{:?} failed: {:?}", website_url, e);
                                        Err(())
                                    }
                                }
                            }))
                        }
                    }
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error reading from stream: {}", e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to Redis: {}", e);
            std::thread::sleep(std::time::Duration::from_secs(1));
            Err(e)
        }
    }
}

#[tokio::main]
async fn main() {
    consume_from_stream();
}

use futures::stream::FuturesUnordered;
use redis::{
    self, Commands, RedisResult,
    streams::{StreamReadOptions, StreamReadReply},
};
use redis_stream::redis_client::RedisStream;
use store::PingOutput;
use tokio::sync::Semaphore;
use uuid::Uuid;
// to send data into the stream for notifier
fn send_alert_to_notifier(
    client: &mut RedisStream,
    website_url: &str,
    reason: &str,
) -> RedisResult<()> {
    let user_email = "gunjanpandey1090@gmail.com";

    println!("Writing to Alert Stream for {}", website_url);

    let _: RedisResult<()> = client.conn.xadd(
        "betteruptime:alerts",
        "*",
        &[
            ("website_url", website_url),
            ("user_email", user_email),
            ("reason", reason),
        ],
    );
    Ok(())
}

// function to handle State Logic
async fn handle_website_check(
    website_id: &str,
    url: &str,
    job_id: String,
    http: Arc<reqwest::Client>,
) -> Result<PingOutput, Error> {
    // 1. Check the last state
    let status_key = format!("betteruptime:status:{}", website_id);
    let last_status: String = client.conn.get(&status_key).unwrap_or("UP".to_string());

    println!("Checking: {} (Last Known Status: {})", url, last_status);

    //Website Ping

    let response = http.get(url).await;

    match response {
        Ok(res) => {
            // if res.status().is_success() {
            //     //CASE: Website is UP
            //     if last_status == "DOWN" {
            //         println!("RECOVERY: {} is back online!", url);
            //         let _: () = client.conn.set(&status_key, "UP").unwrap();
            //     } else {
            //         println!("{} is Healthy.", url);
            //     }
            // } else {
            //     // CASE: Website Error (404, 500 etc)
            //     let reason = format!("Status Code: {}", res.status());
            //     process_failure(client, website_id, url, &reason, &last_status, &status_key);
            // }
            Ok(PingOutput {
                job_id,
                url,
                website_id,
                response,
            })
        }
        Err(e) => {
            // CASE: Website is not Connecting (DNS, Timeout)
            // let reason = e.to_string();
            // process_failure(client, website_id, url, &reason, &last_status, &status_key);
            Err(e)
        }
    }
}

// Failure Logic
fn process_failure(
    client: &mut RedisStream,
    _website_id: &str,
    url: &str,
    reason: &str,
    last_status: &str,
    status_key: &str,
) {
    if last_status == "UP" {
        // First time down
        println!("State Change: UP -> DOWN. Sending Email Alert for {}!", url);

        // 1. set status DOWN
        let _: () = client.conn.set(status_key, "DOWN").unwrap();

        // 2. tell the nnotifier
        let _ = send_alert_to_notifier(client, url, reason);
    } else {
        // Already Down
        println!("{} is still down.", url);
    }
}

pub async fn process_jobs() -> RedisResult<()> {
    dotenvy::dotenv().ok();

    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL");
    let mut client = RedisStream::new(&redis_url)?;

    client.add_consumer()?; 

    let consumer_name = format!("worker-{}", Uuid::new_v4());
    println!("Consumer started as {}", consumer_name);

    let http = Arc::new(reqwest::Client::new());
    let semaphore = Arc::new(Semaphore::new(500));
    let mut futures = FuturesUnordered::new();

    loop {
        // fetch more jobs from redis
        let reply = client.x_read_group(&consumer_name);

        if let Ok(reply) = reply {
            for stream in reply.keys {
                for job in stream.ids {
                    let website_id = match job.map.get("website_id") {
                        Some(redis::Value::Data(v)) => String::from_utf8_lossy(v).to_string(),
                        _ => continue,
                    };

                    let url = match job.map.get("url") {
                        Some(redis::Value::Data(v)) => String::from_utf8_lossy(v).to_string(),
                        _ => continue,
                    };

                    let job_id = job.id.clone();
                    let http = http.clone();
                    let permit = semaphore.clone().acquire_owned().await.unwrap();

                    futures.push(async move {
                        let _permit = permit;

                        let result =
                            handle_website_check(&website_id, &url, job_id.clone(), http).await;

                        PingOutput {
                            job_id,
                            website_id,
                            url,
                            response: result.map(|v| v.response).unwrap_or_else(|e| Err(e)),
                        }
                    });
                }
            }
        }

        // drain completed jobs without blocking new reads
        while let Some(res) = futures.next().await {
            let ping = res;

            match ping.response {
                Ok(_) => {
                    let _ =
                        client
                            .conn
                            .xack("betteruptime:website", "uptime-checkers", &[ping.job_id]);
                }
                Err(e) => {
                    let reason = e.to_string();
                    let status_key = format!("betteruptime:status:{}", ping.website_id);
                    let last_status: String =
                        client.conn.get(&status_key).unwrap_or("UP".to_string());

                    process_failure(
                        &mut client,
                        &ping.website_id,
                        &ping.url,
                        &reason,
                        &last_status,
                        &status_key,
                    );
                }
            }
        }
    }
}

fn main() {
    if let Err(e) = process_jobs() {
        eprintln!("Fatal Error: {}", e);
    }
}

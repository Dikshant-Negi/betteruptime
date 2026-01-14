use redis::{self, Commands, RedisResult, streams::{StreamReadOptions, StreamReadReply}};
use redis_stream::redis_client::RedisStream;
use uuid::Uuid;

// to send data into the stream for notifier
fn send_alert_to_notifier(client: &mut RedisStream, website_url: &str, reason: &str) -> RedisResult<()> {

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
fn handle_website_check(client: &mut RedisStream, website_id: &str, url: &str) {
    // 1. Check the last state 
    let status_key = format!("betteruptime:status:{}", website_id);
    let last_status: String = client.conn.get(&status_key).unwrap_or("UP".to_string());

    println!("Checking: {} (Last Known Status: {})", url, last_status);

    //Website Ping 

    let response = reqwest::blocking::get(url);

    match response {
        Ok(res) => {
            if res.status().is_success() {
                //CASE: Website is UP
                if last_status == "DOWN" {
                    println!("RECOVERY: {} is back online!", url);
                    let _: () = client.conn.set(&status_key, "UP").unwrap();
                } else {
                    println!("{} is Healthy.", url);
                }
            } else {
                // CASE: Website Error (404, 500 etc)
                let reason = format!("Status Code: {}", res.status());
                process_failure(client, website_id, url, &reason, &last_status, &status_key);
            }
        }
        Err(e) => {
            // CASE: Website is not Connecting (DNS, Timeout)
            let reason = e.to_string();
            process_failure(client, website_id, url, &reason, &last_status, &status_key);
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
    status_key: &str
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

pub fn process_jobs() -> RedisResult<()> {
    dotenvy::dotenv().ok();
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL");
    let mut client = RedisStream::new(&redis_url)?;

    let _: RedisResult<()> = client.conn.xgroup_create_mkstream("betteruptime:website", "uptime-checkers", "$");
    
    let consumer_name = format!("worker-{}", Uuid::new_v4());
    println!("Consumer started as {}. Waiting for jobs...", consumer_name);

    loop {
        // Read jobs from Redis Stream
        let options = StreamReadOptions::default()
            .group("uptime-checkers", &consumer_name)
            .count(1)
            .block(2000); // wait for 2 sec if data not found

        let result: RedisResult<StreamReadReply> = client.conn.xread_options(
            &["betteruptime:website"], 
            &[">"], 
            options
        );

        match result {
            Ok(opts) => {
                for entry in opts.keys {
                    for job in entry.ids {
                        //extract data
                        let website_id_val = job.map.get("website_id");
                        let url_val = job.map.get("url");

                        if let (Some(redis::Value::Data(wid)), Some(redis::Value::Data(u))) = (website_id_val, url_val) {
                            let website_id = String::from_utf8(wid.to_vec()).unwrap();
                            let url = String::from_utf8(u.to_vec()).unwrap();

                            // call the main Logic
                            handle_website_check(&mut client, &website_id, &url);

                            // Acknowledge Job
                            let _: RedisResult<()> = client.conn.xack("betteruptime:website", "uptime-checkers", &[job.id]);
                        }
                    }
                }
            }
            Err(e) => {
                if let Some(code) = e.code() {
                    if code != "NOGROUP" {
                        // eprintln!("Redis Error: {}", e);
                    }
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
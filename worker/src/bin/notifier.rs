use lettre::{Message, SmtpTransport, Transport, transport::smtp::{authentication::Credentials}};
use redis::{
    Commands, RedisResult,
    streams::{StreamReadOptions, StreamReadReply},
};
use redis_stream::redis_client::RedisStream;
use tokio;
use uuid::Uuid;
use chrono;

async fn send_email_via_smtp(target_email: &str, website_url: &str, reason: &str) ->bool {
    println!("Sending Email To {}...", target_email);

    let smtp_email = std::env::var("SMTP_EMAIL").expect("email");
    let smtp_password = std::env::var("SMTP_PASSWORD").expect("password");

    let email = Message::builder()
        .from("Uptime Bot <pandey@uptime.com>".parse().unwrap())
        .to(target_email.parse().unwrap())
        .subject(format!("DOWN ALERT: {}", website_url))
        .body(format!(
            "Your Website {} is down. \nReason: {}",
            website_url, reason
        ))
        .unwrap();
    let creds = Credentials::new(smtp_email.to_string(), smtp_password.to_string());

    // Connect to gmail server
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // send the mail
    match tokio::task::spawn_blocking(move || mailer.send(&email)).await {
        Ok(Ok(_)) => {
            println!("Email sent successfully!");
            true
        },
        _ => {
            eprintln!("Email sending Failed.");
            false
        }
    }
}

pub async fn consume_alerts() -> RedisResult<()> {
    dotenvy::dotenv().ok();
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL");
    let consumer_name = format!("notifier-{}", Uuid::new_v4());

    println!("notifier connecting to Redis...");
    let mut client = RedisStream::new(&redis_url)?;

    // creating a new stream "betteruptime:alerts" with the group name of "alert-checker"
    let _: RedisResult<()> =
        client
            .conn
            .xgroup_create_mkstream("betteruptime:alerts", "alert-checker", "$");

    println!("Notifier Connected..");
    println!("Notifier Service Started. Listening for alerts...");

    loop {
        let options = StreamReadOptions::default()
            .group("alert-checker", &consumer_name)
            .count(10)
            .block(2000);

        let result: RedisResult<StreamReadReply> =
            client
                .conn
                .xread_options(&["betteruptime:alerts"], &[">"], options);

        match result {
            Ok(opts) => {
                for entry in opts.keys {
                    for job in entry.ids {
                        // extract data from the stream
                        let url_val = job.map.get("website_url");
                        let email_val = job.map.get("user_email");
                        let reason_val = job.map.get("reason");

                        if let (
                            Some(redis::Value::Data(u)),
                            Some(redis::Value::Data(e)),
                            Some(redis::Value::Data(r)),
                        ) = (url_val, email_val, reason_val)
                        {
                            let url = String::from_utf8(u.to_vec()).unwrap();
                            let email = String::from_utf8(e.to_vec()).unwrap();
                            let reason = String::from_utf8(r.to_vec()).unwrap();

                            println!("Alert Received: {} is DOWN! Reason: {}", url, reason);
                            
                            // call the function to send the email
                            let success = send_email_via_smtp(&email, &url, &reason).await;

                            if success {
                                let _: ()= client.conn.xack("betteruptime:alerts", "alert-checker", &[job.id.as_str()]).unwrap_or_default();
                                let _: () = client.conn.xdel("betteruptime:alerts", &[&job.id]).unwrap_or_default();
                            } else {
                                let retry_at = chrono::Utc::now().timestamp() + 60;
                                let retry_data = format!("{}|{}|{}", email, url, reason);

                                println!("Scheduling retry for {} in 60 seconds...", email);

                                let _: () = client.conn.zadd("betteruptime:retry_alerts", retry_data, retry_at).unwrap_or_default();
                                let _: () = client.conn.xack(("betteruptime:alert", ), "alert-checker", &[&job.id]).unwrap_or_default();

                            }
                        }
                    }
                }
            }
            Err(e) => {
                if let Some(code) = e.code() {
                    if code != "NOGROUP" {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    }
}
async fn start_retry_worker(redis_url: String){
    let mut client = RedisStream::new(&redis_url).unwrap();
    println!("Retrying...");

    loop {
        let now = chrono::Utc::now().timestamp();

        let pending: Vec<String> = client.conn.zrangebyscore("betteruptime:retry_alerts", "-inf", now).unwrap_or_default();

        for item in pending {
            let parts: Vec<&str> = item.split('|').collect();
            if parts.len() == 3 {
                println!("Retrying failed email for {}...", parts[0]);
                if send_email_via_smtp(parts[0], parts[1], parts[2]).await {
                    // If successful now, remove from retry set
                    let _: () = client.conn.zrem("betteruptime:retry_alerts", &item).unwrap_or_default();
                } else {
                    // If failed again, update timestamp to try in another 5 minutes
                    let next_try = chrono::Utc::now().timestamp() + 300;
                    let _: () = client.conn.zadd("betteruptime:retry_alerts", &item, next_try).unwrap_or_default();
                    println!("Retry failed again. Rescheduled for 5 mins later.");
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL");
    
    let url_clone = redis_url.clone();
    tokio::spawn(async move {
        start_retry_worker(url_clone).await;
    });
    if let Err(e) = consume_alerts().await {
        eprintln!("Error: {}", e);
    }
}
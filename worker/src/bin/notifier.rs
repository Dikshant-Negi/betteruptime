use futures::{future, stream::FuturesUnordered};
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use redis::{
    Commands, RedisResult,
    streams::{StreamReadOptions, StreamReadReply},
};
use redis_stream::redis_client::RedisStream;
use std::{fmt::format, option, time::Duration};
use tokio;
use uuid::Uuid;

async fn send_email_via_smtp(target_email: &str, website_url: &str, reason: &str) {
    println!("Sending Email To {}...", target_email);

    // Sender's Email and Password.

    let smtp_email = "pandeyg9010@gmail.com";
    let smtp_password = "sbpn zzgi eobf hiky";

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

    //send the mail
    match tokio::task::spawn_blocking(move || mailer.send(&email)).await {
        Ok(Ok(_)) => println!("Email sent successfully!"),
        Ok(Err(e)) => eprintln!("Email API Failed: {:?}", e),
        Err(e) => eprintln!("Task Join Error {:?}", e),
    }
}

pub async fn consume_alerts() -> RedisResult<()> {
    dotenvy::dotenv().ok();
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL");
    let consumer_name = format!("notifier-{}", Uuid::new_v4());

    println!("notifier connecting to Redis...");
    let mut client = RedisStream::new(&redis_url)?;

    //creating a new stream "betteruptime:alerts" with th group name of "alert-checker"
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
                            send_email_via_smtp(&email, &url, &reason).await;
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

#[tokio::main]
async fn main() {
    if let Err(e) = consume_alerts().await {
        eprintln!("Error: {}", e);
    }
}

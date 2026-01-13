use futures::{future, stream::FuturesUnordered};
use redis::RedisResult;
use redis_stream::redis_client::RedisStream;
use tokio;
use uuid::Uuid;
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use std::{fmt::format, time::Duration};


async fn send_email_via_smtp(target_email: &str, website_url: &str, reason: &str){
    println!("Processing Email Task For {}", website_url);

    // Sender's Email and Password.

    let smtp_email = "mymail";
    let smtp_password= "password";

    let email = Message::builder()
        .from("Uptime Bot <myemail>".parse().unwrap())
        .to(target_email.parse().unwrap())
        .subject(format!("DOWN ALERT: {}", website_url))
        .body(format!("Your Website {} is down. \nReason: {}", website_url, reason))
        .unwrap();
    let creds = Credentials::new(smtp_email.to_string(), smtp_password.to_string());

    // Connect to gmail server
    let mailer = SmtpTransport::relay("smtp.gamail.com")
    .unwrap()
    .credentials(creds)
    .build();

    //send the mail
    match mailer.send(&email) {
        Ok(_) => println!("Email sent to {}", target_email),
        Err(e) => eprintln!("Emailed Failed {:?}", e),
    }
}

pub async fn consume_alerts() -> RedisResult<()> {
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL");

    //genrating unique ID for every notifier
    let consumer_name = format!("notifier-{}", Uuid::new_v4());

    // conecting to redis
    match RedisStream::new(&redis_url){
        Ok(mut client) =>{
            // read from the stream

            match client.x_read_group((&consumer_name.as_str())) {
                Ok(jobs) =>{
                    let futures = FuturesUnordered::new();
                    for entry in jobs.keys {
                        for job in entry.ids {
                            futures.push(tokio::spawn(async move {

                                //extract data
                                let url_val = job.map.get("website_url");
                                let email_val = job.map.get("user_email");
                                let reason_val = job.map.get("reason");

                                //check data is present
                                if let (Some(redis::Value::Data(u)), Some(redis::Value::Data(e)), Some(redis::Value::Data(r))) = (url_val, email_val, reason_val) {
                                    let url = String::from_utf8(u.to_vec()).unwrap();
                                    let email = String::from_utf8(e.to_vec()).unwrap();
                                    let reason = String::from_utf8(r.to_vec()).unwrap();

                                    //call the email function
                                    send_email_via_smtp(&email, &url, &reason).await;
                                } else {
                                    eprintln!("Invalid data in alert job");
                                }
                            }));
                        }
                    }
                    Ok(())
                }
                Err(e) => Err(e)
            }
        }
        Err(e) => {
            eprintln!("Notifier failed to connect: {}", e);
            std::thread::sleep(Duration::from_secs(1));
            Err(e)
        }
    }
}

#[tokio::main]

async fn main() {
    println!("Notifier Service Started...");

    loop {
        let _ = consume_alerts().await;

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
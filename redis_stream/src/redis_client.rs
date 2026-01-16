use chrono::Utc;
use redis::{
    self, Commands, Connection, RedisResult, streams::{StreamReadOptions, StreamReadReply}
};

pub struct RedisStream {
    pub conn: Connection,
}

impl RedisStream {
    pub fn new(url: &str) -> RedisResult<Self> {
        let client = redis::Client::open(url)?;
        let conn = client.get_connection()?;
        Ok(Self { conn })
    }

    // UPDATED: Added 'email' parameter to store user contact info
    pub fn schedule_website(
        &mut self,
        id: &str,
        website_url: &str,
        interval_sec: i32, // Changed to i32 to match your handler logic usually, or keep Option if you prefer
        email: &str,       // <--- NEW PARAMETER
    ) -> RedisResult<()> {
        let now = Utc::now().timestamp();

        let _: RedisResult<()> = self.conn.zadd("betteruptime:schedule", id, now);
        
        let key = format!("betteruptime:site:{}", id);
        
        // Handling interval logic
        let interval = interval_sec.to_string();

        let _: RedisResult<()> = self.conn.hset_multiple(
            &key,
            &[
                ("id", id),
                ("url", website_url),
                ("interval", interval.as_str()),
                ("email", email), // <--- Storing Email in Redis Hash
            ],
        );

        Ok(())
    }

    pub fn process_due_websites(&mut self) -> RedisResult<()> {
        let now = Utc::now().timestamp();
        
        let due_websites: Vec<String> = self.conn.zrangebyscore(
            "betteruptime:schedule",
            "-inf",
            now,
        )?;

        for website_id in due_websites {
            let key = format!("betteruptime:site:{}", website_id);
            
            // Fetching existing data
            let website_url: String = self.conn.hget(&key, "url")?;
            let interval: u64 = self.conn.hget(&key, "interval")?;
            let id: String = self.conn.hget(&key, "id")?;
            
            // NEW: Fetching Email from Hash
            let email: String = self.conn.hget(&key, "email")?; 

            // Passing email to the stream function
            self.x_add(&id, &website_id, &website_url, &email)?;

            // removing the website from schedule and reschedule it with next timestamp
            let _: i64 = self.conn.zrem("betteruptime:schedule", &website_id)?;

            let next_time = now + interval as i64;
            let _: RedisResult<()> = self.conn.zadd("betteruptime:schedule", &website_id, next_time);
        }

        Ok(())
    }

    pub fn add_consumer(&mut self) -> RedisResult<()> {
        let result: RedisResult<()> =
            self.conn
                .xgroup_create_mkstream("betteruptime:website", "uptime-checkers", "$");

        match result {
            Ok(_) => println!("Consumer group created!"),
            Err(err) => println!("Group already exists: {}", err),
        }

        Ok(())
    }

    // UPDATED: Added 'email' parameter to send it to the Consumer
    pub fn x_add(&mut self, id: &str, website_id: &str, website: &str, email: &str) -> RedisResult<()> {
        
        let _: RedisResult<()> =
            self.conn
                .xadd("betteruptime:website", "*", &[
                    ("id", id),
                    ("website_id", website_id),
                    ("url", website),
                    ("email", email), // <--- Adding Email to the Job Card
                ]);

        Ok(())
    }

    pub fn x_read_group(&mut self, consumer_name: &str) -> RedisResult<StreamReadReply> {
        let options = StreamReadOptions::default()
            .group("uptime-checkers", consumer_name)
            .count(10)
            .block(0);

        let reply: StreamReadReply =
            self.conn
                .xread_options(&["betteruptime:website"], &[">"], options)?;

        Ok(reply)
    }
}
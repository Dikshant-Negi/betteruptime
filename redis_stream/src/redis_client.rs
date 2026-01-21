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

    pub fn schedule_website(
        &mut self,
        id: &str,
        website_url: &str,
        interval_sec: i32, 
        user_id: &str,      
    ) -> RedisResult<()> {
        let now = Utc::now().timestamp();

        let _: RedisResult<()> = self.conn.zadd("betteruptime:schedule", id, now);
        
        let key = format!("betteruptime:site:{}", id);
        
        let interval = interval_sec.to_string();

        let _: RedisResult<()> = self.conn.hset_multiple(
            &key,
            &[
                ("id", id),
                ("url", website_url),
                ("interval", interval.as_str()),
                ("user_id", user_id), 
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
            
            //Use Option<> to handle missing data gracefully
            let website_url: Option<String> = self.conn.hget(&key, "url")?;
            let interval: Option<u64> = self.conn.hget(&key, "interval")?;
            let id: Option<String> = self.conn.hget(&key, "id")?;
            let user_id: Option<String> = self.conn.hget(&key, "user_id")?;

            // Only proceed if we have the critical data (URL and UserID)
            if let (Some(url), Some(uid)) = (website_url, user_id) {
                let actual_id = id.unwrap_or(website_id.clone());
                
                // Add to stream
                self.x_add(&actual_id, &website_id, &url, &uid)?;

                // Reschedule
                let _: i64 = self.conn.zrem("betteruptime:schedule", &website_id)?;
                let interval_val = interval.unwrap_or(60);
                let next_time = now + interval_val as i64;
                let _: RedisResult<()> = self.conn.zadd("betteruptime:schedule", &website_id, next_time);
            } else {
                //If data is missing, remove it from the schedule
                println!("Found corrupted data for {}, removing from schedule.", website_id);
                let _: i64 = self.conn.zrem("betteruptime:schedule", &website_id)?;
            }
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

    pub fn x_add(&mut self, id: &str, website_id: &str, website: &str, user_id: &str) -> RedisResult<()> {
        
        let _: RedisResult<()> =
            self.conn
                .xadd("betteruptime:website", "*", &[
                    ("id", id),
                    ("website_id", website_id),
                    ("url", website),
                    ("user_id", user_id),
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
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

    // used sorted set to schedule my website to insert them in redis stream based on timestamp.
    pub fn schedule_website(
        &mut self,
        id: &str,
        website_url: &str,
        interval_sec: Option<i32>,
    ) -> RedisResult<()> {
        let now = Utc::now().timestamp();

        let _:RedisResult<()> = self.conn.zadd("betteruptime:schedule", id, now);
        
        let key = format!("betteruptime:site:{}", id);
        let interval = match interval_sec{
            Some(i)=>i.to_string(),
            None=>"60".to_string(),
        };
        let _:RedisResult<()> = self.conn.hset_multiple(
            &key,
            &[
                ("id",id),
                ("url", website_url),
                ("interval",interval.as_str()),
            ],
        );

        Ok(())
    }

    pub fn 
    process_due_websites(&mut self)->RedisResult<()>{
        let now = Utc::now().timestamp();
        
        let due_websites: Vec<String> = self.conn.zrangebyscore(
            "betteruptime:schedule",
            "-inf",
            now,
        )?;

        for website_id in due_websites {
            let key = format!("betteruptime:site:{}", website_id);
            let website_url: String = self.conn.hget(&key, "url")?;
            let interval: u64 = self.conn.hget(&key, "interval")?;
            let id:String = self.conn.hget(&key,"id")?;
            self.x_add(&id,&website_id, &website_url)?;

            // removing the website from schedule and reschedule it with next timestamp
            let _ :i64 = self.conn.zrem("betteruptime:schedule", &website_id)?;

            let next_time = now + interval as i64;
            let _:RedisResult<()> = self.conn.zadd("betteruptime:schedule", &website_id, next_time);
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

    pub fn x_add(&mut self, id: &str, website_id:&str,website: &str) -> RedisResult<()> {
        
        let _: RedisResult<()> =
            self.conn
                .xadd("betteruptime:website", "*", &[
                    ("id", id),
                    ("website_id", website_id),
                    ("url", website),
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

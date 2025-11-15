use redis::{
    self, Commands, Connection, RedisResult,
    streams::{StreamReadOptions, StreamReadReply},
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

    pub fn x_add(&mut self, id: &str, website: &str) -> RedisResult<()> {
        let payload = serde_json::json!({
            "id": id,
            "website": website
        }).to_string();

        let res :RedisResult<()> = self.conn.xadd("betteruptime:website", "*", &[("payload",payload)]);

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

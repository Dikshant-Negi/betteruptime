use redis::{self, RedisResult, Connection};


pub struct RedisStream {
    pub conn: Connection,
}

impl RedisStream {
    pub fn new(url: &str) -> RedisResult<Self> {

        let client = redis::Client::open(url)?;
        let conn = client.get_connection()?;

        Ok(Self { conn })
    }

    pub fn x_add(&mut self,id:&str,website:&str)->RedisResult<()>{
        let payload = serde_json::json!({
            "id": id,
            "website": website
        });

        self::conn.xadd("betteruptime:website","*",payload);

        Ok(())
    }
}

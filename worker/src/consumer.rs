use redis::RedisResult;
use redis_stream::redis_client::RedisStream;
use uuid::Uuid;
use futures::stream::{FuturesUnordered, StreamExt};
use tokio;

fn inert_into_stream() -> RedisResult<()> {
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL");
    let id = Uuid::new_v4();
    let consumer_name = formate!("consumer-{}",id);
    match RedisStream::new(&redis_url) {
        Ok(mut client) => {
           match client.x_read_group(consumer_name){
            Ok(jobs)=>{
                let mut futures = FeaturesUnordered::new();
                for job in jobs{
                    futures.push(tokio::spawn(async move{
                        println!(job);
                    }))
                }
            }
           }
           Err(_)=>{

           }

        }
        Err(e) => {
            eprintln!("Failed to connect to Redis: {}", e);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}

#[tokio::main]
fn main(){
    insert_into_stream();
}


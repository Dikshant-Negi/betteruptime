    use crate::store::Store;
    use  uuid::Uuid;
    pub struct Output{
        pub id:String,
        pub url:String,
        pub check_interval:Option<i32>
    }

    impl Store{
        pub async fn create_websites(&self,url:String,user_id:String,name:String,check_interval:i32)->Result<Output,sqlx::Error>{
            let id = Uuid::new_v4().to_string();
            let res: Result<_, sqlx::Error> = sqlx::query!("INSERT INTO websites (id,user_id,name,url,check_interval) VALUES ($1,$2,$3,$4,$5) RETURNING id,url,check_interval", id, user_id, name, url, check_interval).fetch_one(&self.conn).await;
            match res{
            Ok(record)=>{
                Ok(Output { id:record.id, url:record.url, check_interval:record.check_interval })

            }
            Err(e)=>{
                Err(e)
            }
        }
        }
    }
use crate::store::Store;
use  uuid::Uuid;
impl Store{
    pub async fn create_website(&self,url:String,user_id:String,name:String,check_interval:i32)->Result<bool,sqlx::Error>{
        let id = Uuid::new_v4().to_string();
        let res = sqlx::query!("INSERT INTO websites (id,user_id,name,url,check_interval) VALUES ($1,$2,$3,$4,$5)", id, user_id, name, url, check_interval).execute(&self.conn).await?;
        Ok(res.rows_affected() >0)
    }
}
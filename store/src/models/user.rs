use uuid::Uuid;
use crate::store::Store;
impl Store{
      pub async fn create_user(&self,email:String,password:String,name:String)->Result<bool,sqlx::Error>{
        let id = Uuid::new_v4().to_string();
        let res = sqlx::query!("INSERT INTO users (id,name,email,password) VALUES ($1,$2,$3,$4)", id, name, email, password).execute(&self.conn).await?;
        Ok(res.rows_affected() > 0)
      }

    pub async fn sigin(&self,email:String,password:String)->Result<String,sqlx::Error>{
        let res = sqlx::query!("SELECT id FROM users WHERE email=$1 AND password=$2",email,password).fetch_one(&self.conn).await;

        match res{
            Ok(res)=>{
                Ok(res.id)
            }
            Err(e)=>{
                Err(e)
            }
        }
    }
}
use uuid::Uuid;
use crate::store::Store;
impl Store{
      pub async fn create_user(&self,email:String,password:String,name:String)->Result<String,sqlx::Error>{
        let id = Uuid::new_v4().to_string();
        let res = sqlx::query!("INSERT INTO users (id,name,email,password) VALUES ($1,$2,$3,$4) RETURNING id", id, name, email, password).fetch_one(&self.conn).await;
        match res{
            Ok(record)=>{
                Ok(record.id)
            }
            Err(e)=>{
                Err(e)
            }
        }
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

    // to fetch user's email id
    pub async fn get_user_email(&self, user_id: &str) -> Result<String, sqlx::Error> {
        let rec = sqlx::query!("SELECT email FROM users WHERE id = $1", user_id)
            .fetch_one(&self.conn)
            .await?;
        
        Ok(rec.email)
    }
}
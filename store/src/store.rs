use sqlx::PgPool;

pub struct Store{
    pub conn : PgPool
}

impl Store{

    pub async fn new(url:&mut String)->Result<Self,sqlx::Error>{
       
       
        let pool = PgPool::connect(url).await;

        match pool{
            Ok(conn)=>{
                Ok(Store{
                    conn
                })
            }
            Err(e)=>{
                Err(e)
            }
        }
    }

  


}

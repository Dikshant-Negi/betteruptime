
struct PingOutput{
    pub job_id:String,
    pub url:String,
    pub website_id:String,
    pub response:anyhow::Result<()>
}
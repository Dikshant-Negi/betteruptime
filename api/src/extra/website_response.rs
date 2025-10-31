use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct WebsiteInput{
    pub url:String,
    pub name:String,
    pub interval:i32
}

#[derive(Serialize,Deserialize)]
pub struct WebsiteOutput{
    pub success:bool,
    pub message:String,

}
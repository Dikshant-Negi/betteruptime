use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
pub struct CreateUserInput{
    pub username:String,
    pub email:String,
    pub password:String
}


#[derive(Serialize,Deserialize)]
pub struct CreateUserOuptput{
    pub success:bool,
    pub jwt:String,
    pub message:String
}

#[derive(Serialize,Deserialize)]
pub struct SignInput{
    pub email:String,
    pub password:String
}
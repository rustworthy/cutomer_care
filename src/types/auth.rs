use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Creds {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub exp: usize,
    pub sub: String,
    pub moderator: bool,
}

#[derive(Serialize, Debug)]
pub struct Token {
    pub token: String,
}

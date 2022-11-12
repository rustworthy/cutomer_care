use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct UserIn {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub is_moderator: Option<bool>,
}

#[derive(Serialize)]
pub struct UserOut {
    pub _id: String,
    pub created_at: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub is_moderator: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
}

#[derive(Deserialize)]
pub struct UserTknDetails {
    pub _id: String,
    pub is_moderator: bool,
}

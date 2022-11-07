use serde::Serialize;

#[derive(Serialize)]
pub struct UserOut {
    pub _id: String,
    pub created_at: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub is_stuff: bool,
    pub is_superuser: bool,
}

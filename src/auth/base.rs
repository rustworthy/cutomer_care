use crate::types::user::UserTknDetails;

pub trait AuthProvider: std::fmt::Debug + Clone + std::marker::Send {
    fn parse_token(&self, tkn: String) -> Option<UserTknDetails>;
    fn issue_token(&self, u: UserTknDetails) -> Option<String>;
}

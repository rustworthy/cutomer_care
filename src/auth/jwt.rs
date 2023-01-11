use std::env::VarError;

use super::base::AuthProvider;
use crate::types::{auth::Claims, user::UserTknDetails};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

const TOKEN_EXP_MINS: i64 = 5;

#[derive(Clone, Debug)]
pub struct JWTAuth {
    secret: String,
}

impl JWTAuth {
    pub fn new() -> Result<Self, VarError> {
        let secret = std::env::var("AUTH_SECRET")?;
        Ok(JWTAuth { secret })
    }
}

impl AuthProvider for JWTAuth {
    fn issue_token(&self, u: UserTknDetails) -> Option<String> {
        let claims = Claims {
            exp: (Utc::now() + Duration::minutes(TOKEN_EXP_MINS)).timestamp() as usize,
            sub: u._id.clone(),
            moderator: u.is_moderator,
        };
        let tkn = encode(&Header::default(), &claims, &EncodingKey::from_secret(self.secret.as_bytes()));
        if tkn.is_err() {
            return None;
        }
        Some(tkn.unwrap())
    }

    fn parse_token(&self, tkn: String) -> Option<UserTknDetails> {
        let tkn_data = decode::<Claims>(
            &tkn.replace("Token ", ""),
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        );
        if tkn_data.is_err() {
            return None;
        }
        let claims = tkn_data.unwrap().claims;
        Some({
            UserTknDetails {
                _id: claims.sub,
                is_moderator: claims.moderator,
            }
        })
    }
}

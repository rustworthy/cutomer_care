use super::base::AuthProvider;
use crate::types::{auth::Claims, user::UserTknDetails};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

const TOKEN_EXP_MINS: i64 = 5;

lazy_static! {
    static ref ENCODING_KEY: EncodingKey =
        EncodingKey::from_rsa_pem(include_bytes!("../../.jwt/jwtkey.pem"))
            .expect("JWT encoding private key missing");
    static ref DECODING_KEY: DecodingKey =
        DecodingKey::from_rsa_pem(include_bytes!("../../.jwt/jwtkey_public.pem"))
            .expect("JET decoding public key missing");
}

#[derive(Clone, Copy, Debug)]
pub struct JWTAuth;

impl AuthProvider for JWTAuth {
    fn issue_token(&self, u: UserTknDetails) -> Option<String> {
        let claims = Claims {
            exp: (Utc::now() + Duration::minutes(TOKEN_EXP_MINS)).timestamp() as usize,
            sub: u._id.clone(),
            moderator: u.is_moderator,
        };
        let tkn = encode(&Header::new(Algorithm::RS256), &claims, &ENCODING_KEY);
        if tkn.is_err() {
            return None;
        }
        Some(tkn.unwrap())
    }

    fn parse_token(&self, tkn: String) -> Option<UserTknDetails> {
        let tkn_data = decode::<Claims>(
            &tkn.replace("Token ", ""),
            &DECODING_KEY,
            &Validation::new(Algorithm::RS256),
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

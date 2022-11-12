use std::future;

use crate::{
    store::base::Db,
    types::{
        auth::{Claims, Creds, Token},
        user::UserTknDetails,
    },
};
use chrono::{Duration, Utc};
use error_handling::ServiceError;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use tracing::instrument;
use warp::{http::StatusCode, Filter, Rejection, Reply};

const TOKEN_EXP_MINS: i64 = 5;

lazy_static! {
    static ref ENCODING_KEY: EncodingKey =
        EncodingKey::from_rsa_pem(include_bytes!("../../.jwt/jwtkey.pem"))
            .expect("JWT encoding private key missing");
    static ref DECODING_KEY: DecodingKey =
        DecodingKey::from_rsa_pem(include_bytes!("../../.jwt/jwtkey_public.pem"))
            .expect("JET decoding public key missing");
}

pub fn moderator_auth() -> impl Filter<Extract = (Option<String>,), Error = warp::Rejection> + Clone
{
    warp::header::optional::<String>("Authorization")
}

pub fn jwt_auth() -> impl Filter<Extract = (UserTknDetails,), Error = warp::Rejection> + Clone {
    warp::header::optional::<String>("Authorization").and_then(|token: Option<String>| {
        if token.is_none() {
            return future::ready(Err(warp::reject::custom(
                ServiceError::AuthTokenMissingOrInvalid,
            )));
        }
        match parse_token(token.unwrap()) {
            None => future::ready(Err(warp::reject::custom(
                ServiceError::AuthTokenMissingOrInvalid,
            ))),
            Some(user_details) => future::ready(Ok(user_details)),
        }
    })
}

fn parse_token(tkn: String) -> Option<UserTknDetails> {
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

fn issue_token(u: UserTknDetails) -> Option<String> {
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

#[instrument]
pub async fn login(db: Db, creds: Creds) -> Result<impl Reply, Rejection> {
    let user_fetched = db.get_user_by_creds(creds).await;
    if let Err(e) = user_fetched {
        return Err(warp::reject::custom(e));
    }
    let u = user_fetched.unwrap();
    let u = UserTknDetails {
        _id: u._id,
        is_moderator: u.is_moderator,
    };
    match issue_token(u) {
        None => Err(warp::reject::custom(ServiceError::AuthTokenEncoderErr)),
        Some(token) => Ok(warp::reply::with_status(
            warp::reply::json(&Token { token }),
            StatusCode::CREATED,
        )),
    }
}

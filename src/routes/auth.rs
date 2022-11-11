use crate::{
    store::base::Db,
    types::{
        auth::{Claims, Creds, Token},
        user::UserOut,
    },
};
use error_handling::ServiceError;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use tracing::instrument;
use warp::{http::StatusCode, Filter, Rejection, Reply};

lazy_static! {
    static ref ENCODING_KEY: EncodingKey =
        EncodingKey::from_rsa_pem(include_bytes!("../../.jwt/jwtkey.pem"))
            .expect("JWT encoding key file missing");
}

pub fn moderator_auth() -> impl Filter<Extract = (Option<String>,), Error = warp::Rejection> + Clone
{
    warp::header::optional::<String>("Authorization")
}

async fn create_token_for_user(u: UserOut) -> Option<String> {
    let claims = Claims {
        exp: 1000000,
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
    match create_token_for_user(user_fetched.unwrap()).await {
        None => Err(warp::reject::custom(ServiceError::JWTEncoderErr)),
        Some(token) => Ok(warp::reply::with_status(
            warp::reply::json(&Token { token }),
            StatusCode::CREATED,
        )),
    }
}

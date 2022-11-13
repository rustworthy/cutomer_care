use std::future;

use crate::{
    auth_providers::base::AuthProvider,
    store::base::Db,
    types::{
        auth::{Creds, Token},
        user::UserTknDetails,
    },
};
use error_handling::ServiceError;
use tracing::instrument;
use warp::{http::StatusCode, Filter, Rejection, Reply};

pub fn parse_auth_headers(
) -> impl Filter<Extract = (Option<String>,), Error = warp::Rejection> + Clone {
    warp::header::optional::<String>("Authorization")
}

pub fn authenticate<T: AuthProvider>(
    auth_provider: T,
) -> impl Filter<Extract = (UserTknDetails,), Error = warp::Rejection> + Clone {
    warp::header::optional::<String>("Authorization").and_then(move |token: Option<String>| {
        if token.is_none() {
            return future::ready(Err(warp::reject::custom(
                ServiceError::AuthTokenMissingOrInvalid,
            )));
        }
        match auth_provider.parse_token(token.unwrap()) {
            None => future::ready(Err(warp::reject::custom(
                ServiceError::AuthTokenMissingOrInvalid,
            ))),
            Some(user_details) => future::ready(Ok(user_details)),
        }
    })
}

#[instrument]
pub async fn login<T: AuthProvider>(
    creds: Creds,
    db: Db,
    auth_provider: T,
) -> Result<impl Reply, Rejection> {
    let u = db
        .get_user_by_creds(creds)
        .await
        .map_err(warp::reject::custom)?;
    let u = UserTknDetails {
        _id: u._id,
        is_moderator: u.is_moderator,
    };
    match auth_provider.issue_token(u) {
        None => Err(warp::reject::custom(ServiceError::AuthTokenEncoderErr)),
        Some(token) => Ok(warp::reply::with_status(
            warp::reply::json(&Token { token }),
            StatusCode::CREATED,
        )),
    }
}

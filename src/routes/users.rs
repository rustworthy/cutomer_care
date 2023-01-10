use crate::{store::base::Db, types::user::UserIn};
use error_handling::ServiceError;
use warp::{http::StatusCode, Rejection, Reply};

pub async fn validate_moderator(
    new_user: UserIn,
    moderator_key_presented: Option<String>,
    moderator_key_real: String,
) -> Result<UserIn, ServiceError> {
    if new_user.is_moderator.unwrap_or(false) {
        let key_presented = moderator_key_presented.unwrap_or_default();
        if key_presented.trim().is_empty() {
            return Err(ServiceError::AuthCredsMissing);
        }
        if !moderator_key_real.eq(&key_presented) {
            return Err(ServiceError::AuthCredsMissing);
        }
    }
    Ok(new_user)
}

pub async fn add_user(
    new_user: UserIn,
    auth_headers: Option<String>,
    db: Db,
    moderator_key: String,
) -> Result<impl Reply, Rejection> {
    let new_user = validate_moderator(new_user, auth_headers, moderator_key)
        .await
        .map_err(warp::reject::custom)?;

    let inserted_id = db.add_user(new_user).await.map_err(warp::reject::custom)?;
    Ok(warp::reply::with_status(
        warp::reply::json(&inserted_id.as_dict()),
        StatusCode::CREATED,
    ))
}

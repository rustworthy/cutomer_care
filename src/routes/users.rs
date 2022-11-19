use crate::{store::base::Db, types::user::UserIn};
use error_handling::ServiceError;
use warp::{http::StatusCode, Rejection, Reply};

pub async fn check_moderator(u: UserIn, key_presented: Option<String>, moderator_key: String) -> Result<UserIn, ServiceError> {
    if u.is_moderator.unwrap_or(false) {
        let key_presented = key_presented.unwrap_or_default();
        if key_presented.trim().is_empty() {
            return Err(ServiceError::AuthCredsMissing);
        }
        if !moderator_key.eq(&key_presented) {
            return Err(ServiceError::AuthCredsMissing);
        }
    }
    Ok(u)
}

pub async fn add_user(usr: UserIn, auth_headers: Option<String>, db: Db, moderator_key: String) -> Result<impl Reply, Rejection> {
    let usr = match check_moderator(usr, auth_headers, moderator_key).await {
        Ok(u) => u,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    match db.add_user(usr).await {
        Ok(id) => Ok(warp::reply::with_status(
            warp::reply::json(&id.as_dict()),
            StatusCode::CREATED,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

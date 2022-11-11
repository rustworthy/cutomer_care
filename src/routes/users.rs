use crate::{store::base::Db, types::user::UserIn};
use error_handling::ServiceError;
use warp::{http::StatusCode, Rejection, Reply};

lazy_static! {
    static ref SECRET_KEY: String = {
        std::env::var("MODERATOR_AUTH_KEY")
            .expect("Environment variable MODERATOR_AUTH_KEY not set.")
    };
}

pub async fn check_moderator(u: UserIn, key: Option<String>) -> Result<UserIn, ServiceError> {
    if u.is_moderator.unwrap_or(false) {
        let key = key.unwrap_or_default();
        if key.trim().is_empty() {
            return Err(ServiceError::AuthCredsMissing);
        }
        if !SECRET_KEY.eq(&key) {
            return Err(ServiceError::AuthCredsMissing);
        }
    }
    Ok(u)
}

pub async fn add_user(db: Db, u: UserIn, key: Option<String>) -> Result<impl Reply, Rejection> {
    let u = match check_moderator(u, key).await {
        Ok(u) => u,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    match db.add_user(u).await {
        Ok(id) => Ok(warp::reply::with_status(
            warp::reply::json(&id.as_dict()),
            StatusCode::CREATED,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

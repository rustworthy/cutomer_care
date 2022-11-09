use warp::{http::StatusCode, Rejection, Reply};

use crate::{store::base::Db, types::user::UserIn};

pub async fn add_user(db: Db, u: UserIn) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::with_status("", StatusCode::CREATED))
}

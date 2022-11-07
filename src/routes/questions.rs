use std::str::FromStr;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

use crate::store::Db;
use crate::types::pagination::Pagination;
use crate::types::question::{QuestId, QuestInput};

type Params = std::collections::HashMap<String, String>;

pub async fn list_guestions(params: Params, db: Db) -> Result<impl Reply, Rejection> {
    let pgn = match params.is_empty() {
        true => Pagination::default(),
        false => Pagination::parse_from_map(params)?,
    };

    match db.list(pgn.offset, pgn.limit).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn add_question(db: Db, q: QuestInput) -> Result<impl Reply, Rejection> {
    match db.add(q).await {
        Ok(inserted_id) => Ok(warp::reply::with_status(
            warp::reply::json(&inserted_id.as_dict()),
            StatusCode::CREATED,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn update_question(id: String, db: Db, q: QuestInput) -> Result<impl Reply, Rejection> {
    match db.update(QuestId::from_str(&id).unwrap(), q).await {
        Ok(_) => Ok(warp::reply::with_status("", StatusCode::NO_CONTENT)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn delete_question(id: String, db: Db) -> Result<impl Reply, Rejection> {
    match db.delete(QuestId::from_str(&id).unwrap()).await {
        Ok(_) => Ok(warp::reply::with_status("", StatusCode::NO_CONTENT)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn get_question(id: String, db: Db) -> Result<impl Reply, Rejection> {
    match db.get(QuestId::from_str(&id).unwrap()).await {
        Ok(quest) => Ok(warp::reply::json(&quest)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

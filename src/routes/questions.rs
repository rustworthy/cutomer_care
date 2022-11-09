use std::str::FromStr;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

use crate::store::base::Db;
use crate::text_processing::filter_out_bad_words;
use crate::types::pagination::Pagination;
use crate::types::question::{QuestId, QuestIn};

type Params = std::collections::HashMap<String, String>;

pub async fn censor_quest(mut q: QuestIn) -> Result<QuestIn, Rejection> {
    let title = tokio::spawn(filter_out_bad_words(q.title));
    let content = tokio::spawn(filter_out_bad_words(q.content));
    let (title, content) = (title.await.unwrap(), content.await.unwrap());
    q.title = match title {
        Ok(sanitized_text) => sanitized_text,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    q.content = match content {
        Ok(sanitized_text) => sanitized_text,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    Ok(q)
}

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

pub async fn add_question(db: Db, q: QuestIn) -> Result<impl Reply, Rejection> {
    let q = censor_quest(q).await?;
    match db.add(q).await {
        Ok(inserted_id) => Ok(warp::reply::with_status(
            warp::reply::json(&inserted_id.as_dict()),
            StatusCode::CREATED,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn update_question(id: String, db: Db, q: QuestIn) -> Result<impl Reply, Rejection> {
    let q = censor_quest(q).await?;
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

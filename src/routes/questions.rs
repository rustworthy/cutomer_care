use error_handling::ServiceError;
use std::str::FromStr;
use warp::http::StatusCode;
use warp::{Rejection, Reply};
use tracing::{event, Level, instrument};

use crate::store::ThreadSafeStore;
use crate::types::pagination::Pagination;
use crate::types::question::{QuestId, QuestInput};

type Params = std::collections::HashMap<String, String>;

#[instrument]
pub async fn list_guestions(params: Params, st: ThreadSafeStore) -> Result<impl Reply, Rejection> {
    event!(Level::INFO, "list_question handler hit");
    let locked_store = st.read();
    let unpaginated_quests = locked_store.all();

    if params.is_empty() {
        event!(Level::INFO, "no query string params -> returning all the questions");
        return Ok(warp::reply::json(&unpaginated_quests));
    }

    let pgn = Pagination::parse_from_map(params)?;
    if pgn.end >= unpaginated_quests.len() {
        event!(Level::INFO, "pagination end value ({}) gt questions count -> returning all the questions", pgn.end);
        return Ok(warp::reply::json(&unpaginated_quests));
    }
    event!(Level::INFO, "returning questions questions from {} to {}", pgn.start, pgn.end);
    let requested_chunk = &unpaginated_quests[pgn.start..pgn.end];
    Ok(warp::reply::json(&requested_chunk))
}

pub async fn add_question(store: ThreadSafeStore, q: QuestInput) -> Result<impl Reply, Rejection> {
    let inserted_id = store.write().save(q);
    Ok(warp::reply::with_status(
        warp::reply::json(&inserted_id.as_dict()),
        StatusCode::CREATED,
    ))
}

pub async fn update_question(
    id: String,
    st: ThreadSafeStore,
    q: QuestInput,
) -> Result<impl Reply, Rejection> {
    if st
        .write()
        .update(QuestId::from_str(&id).unwrap(), q)
        .is_ok()
    {
        return Ok(warp::reply::with_status("", StatusCode::NO_CONTENT));
    }
    Err(warp::reject::custom(ServiceError::ObjectNotFound))
}

pub async fn delete_question(id: String, st: ThreadSafeStore) -> Result<impl Reply, Rejection> {
    if st.write().remove(QuestId::from_str(&id).unwrap()).is_some() {
        return Ok(warp::reply::with_status("", StatusCode::NO_CONTENT));
    }
    Err(warp::reject::custom(ServiceError::ObjectNotFound))
}

pub async fn get_question(id: String, st: ThreadSafeStore) -> Result<impl Reply, Rejection> {
    if let Some(quest) = st.read().one(QuestId::from_str(&id).unwrap()) {
        return Ok(warp::reply::json(quest));
    }
    Err(warp::reject::custom(ServiceError::ObjectNotFound))
}

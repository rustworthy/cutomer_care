use error_handling::ServiceError;
use parking_lot::RwLock;
use std::str::FromStr;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

use crate::store::Store;
use crate::types::pagination::Pagination;
use crate::types::question::{QuestId, QuestInput, Question};

type ArcStore = Arc<RwLock<Store>>;
type Params = std::collections::HashMap<String, String>;

pub async fn list_guestions(
    params: Params,
    st: ArcStore,
) -> Result<impl warp::Reply, warp::Rejection> {
    let unpaginated_quests: Vec<Question> = st.read().questions.values().cloned().collect();
    if params.is_empty() {
        return Ok(warp::reply::json(&unpaginated_quests));
    }
    let pgn = Pagination::parse_from_map(params)?;
    if pgn.end >= unpaginated_quests.len() {
        return Ok(warp::reply::json(&unpaginated_quests));
    }
    let requested_chunk = &unpaginated_quests[pgn.start..pgn.end];
    Ok(warp::reply::json(&requested_chunk))
}

pub async fn add_question(store: ArcStore, q: QuestInput) -> Result<impl Reply, Rejection> {
    let q = q.prepare_for_storage();
    store.write().questions.insert(q.id.clone(), q);
    Ok(warp::reply::with_status(
        "Question successfully added",
        StatusCode::CREATED,
    ))
}

pub async fn update_question(
    id: String,
    st: ArcStore,
    q: QuestInput,
) -> Result<impl Reply, Rejection> {
    if let Some(quest) = st
        .write()
        .questions
        .get_mut(&QuestId::from_str(&id).unwrap())
    {
        quest.title = q.title;
        quest.content = q.content;
        quest.tags = q.tags;
        return Ok(warp::reply::with_status("", StatusCode::NO_CONTENT));
    }
    Err(warp::reject::custom(ServiceError::ObjectNotFound))
}

pub async fn delete_question(
    id: String,
    store: Arc<RwLock<Store>>,
) -> Result<impl Reply, Rejection> {
    match store
        .write()
        .questions
        .remove(&QuestId::from_str(&id).unwrap())
    {
        Some(_) => Ok(warp::reply::with_status("", StatusCode::NO_CONTENT)),
        None => Err(warp::reject::custom(ServiceError::ObjectNotFound)),
    }
}

pub async fn get_question(id: String, store: Arc<RwLock<Store>>) -> Result<impl Reply, Rejection> {
    match store.read().questions.get(&QuestId::from_str(&id).unwrap()) {
        Some(quest) => Ok(warp::reply::json(&quest)),
        None => Err(warp::reject::custom(ServiceError::ObjectNotFound)),
    }
}

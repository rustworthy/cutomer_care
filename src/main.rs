use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use std::sync::Arc;
use warp::body::BodyDeserializeError;
use warp::cors::CorsForbidden;
use warp::reject::Reject;
use warp::{http, Filter};
use warp::{Rejection, Reply};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
struct QuestionId(String);

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
            false => Ok(QuestionId(id.to_string())),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug)]
enum ServiceError {
    ParseError(std::num::ParseIntError),
    MissingParams,
    InvalidParamsRange,
    ObjectNotFound,
}

impl Reject for ServiceError {}
impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(err) => write!(f, "Failed to parse parameter: {}", err),
            Self::MissingParams => write!(f, "Missing parameter"),
            Self::InvalidParamsRange => write!(f, "Invalid parameters range"),
            Self::ObjectNotFound => write!(f, "Not found"),
        }
    }
}

struct Pagination {
    start: usize,
    end: usize,
}

impl Pagination {
    fn parse_from_map(params: HashMap<String, String>) -> Result<Self, ServiceError> {
        if !params.contains_key("start") || !params.contains_key("end") {
            return Err(ServiceError::MissingParams);
        }

        let start = params
            .get("start")
            .unwrap()
            .parse::<usize>()
            .map_err(ServiceError::ParseError)?;
        let end = params
            .get("end")
            .unwrap()
            .parse::<usize>()
            .map_err(ServiceError::ParseError)?;

        if start > end {
            return Err(ServiceError::InvalidParamsRange);
        }

        Ok(Self { start, end })
    }
}

async fn list_guestions(
    params: HashMap<String, String>,
    store: Arc<RwLock<Store>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let unpaginated_quests: Vec<Question> = store.read().questions.values().cloned().collect();
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

async fn add_question(store: Arc<RwLock<Store>>, quest: Question) -> Result<impl Reply, Rejection> {
    store.write().questions.insert(quest.id.clone(), quest);
    Ok(warp::reply::with_status(
        "Question successfully added",
        http::StatusCode::CREATED,
    ))
}

async fn update_question(
    id: String,
    store: Arc<RwLock<Store>>,
    quest_upd: Question,
) -> Result<impl Reply, Rejection> {
    match store.write().questions.get_mut(&QuestionId(id)) {
        Some(quest) => *quest = quest_upd,
        None => return Err(warp::reject::custom(ServiceError::ObjectNotFound)),
    }
    Ok(warp::reply::with_status("", http::StatusCode::NO_CONTENT))
}

async fn delete_question(id: String, store: Arc<RwLock<Store>>) -> Result<impl Reply, Rejection> {
    match store.write().questions.remove(&QuestionId(id)) {
        Some(_) => Ok(warp::reply::with_status("", http::StatusCode::NO_CONTENT)),
        None => Err(warp::reject::custom(ServiceError::ObjectNotFound)),
    }
}

async fn get_question(id: String, store: Arc<RwLock<Store>>) -> Result<impl Reply, Rejection> {
    match store.read().questions.get(&QuestionId(id)) {
        Some(quest) => Ok(warp::reply::json(&quest)),
        None => Err(warp::reject::custom(ServiceError::ObjectNotFound)),
    }
}

async fn handle_err(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(err) = r.find::<CorsForbidden>() {
        return Ok(warp::reply::with_status(
            err.to_string(),
            http::StatusCode::FORBIDDEN,
        ));
    }

    if let Some(err) = r.find::<ServiceError>() {
        return Ok(warp::reply::with_status(
            err.to_string(),
            http::StatusCode::RANGE_NOT_SATISFIABLE,
        ));
    }

    if let Some(err) = r.find::<BodyDeserializeError>() {
        return Ok(warp::reply::with_status(
            err.to_string(),
            http::StatusCode::UNPROCESSABLE_ENTITY,
        ));
    };

    println!("{:?}", r);
    Ok(warp::reply::with_status(
        "Not found".to_string(),
        http::StatusCode::NOT_FOUND,
    ))
}

#[derive(Clone)]
struct Store {
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Self::init(),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("Failed to read quetions id")
    }
}

#[tokio::main]
async fn main() {
    let store = Arc::new(RwLock::new(Store::new()));
    let store_filter = warp::any().map(move || Arc::clone(&store));
    let cors = warp::cors()
        .allow_methods(vec![http::Method::PUT, http::Method::DELETE])
        .allow_origins(vec!["http://front-end-service:3000"])
        .allow_header("content-type");

    let list_quest = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(list_guestions);

    let add_quest = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_question);

    let upd_quest = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(update_question);

    let del_quest = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_question);

    let get_quest = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_question);

    let routes = list_quest
        .or(add_quest)
        .or(upd_quest)
        .or(del_quest)
        .or(get_quest)
        .with(cors)
        .recover(handle_err);

    warp::serve(routes).run(([127, 0, 0, 1], 7878)).await;
}

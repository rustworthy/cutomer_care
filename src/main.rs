use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;
use warp::body::BodyDeserializeError;
use warp::cors::CorsForbidden;
use warp::reject::Reject;
use warp::{http, Filter};
use warp::{Rejection, Reply};

type ArcStore = Arc<RwLock<Store>>;
type Params = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
struct QuestId(String);
impl QuestId {
    fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl FromStr for QuestId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
            false => Ok(QuestId(id.to_string())),
        }
    }
}

#[derive(Debug, Deserialize)]
struct QuestInput {
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
impl QuestInput {
    fn prepare_for_storage(self) -> Question {
        Question {
            id: QuestId::new(),
            title: self.title,
            content: self.content,
            tags: self.tags,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Question {
    id: QuestId,
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

async fn list_guestions(params: Params, st: ArcStore) -> Result<impl warp::Reply, warp::Rejection> {
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

async fn add_question(store: ArcStore, q: QuestInput) -> Result<impl Reply, Rejection> {
    let q = q.prepare_for_storage();
    store.write().questions.insert(q.id.clone(), q);
    Ok(warp::reply::with_status(
        "Question successfully added",
        http::StatusCode::CREATED,
    ))
}

async fn update_question(id: String, st: ArcStore, q: QuestInput) -> Result<impl Reply, Rejection> {
    if let Some(quest) = st.write().questions.get_mut(&QuestId(id)) {
        quest.title = q.title;
        quest.content = q.content;
        quest.tags = q.tags;
        return Ok(warp::reply::with_status("", http::StatusCode::NO_CONTENT));
    }
    Err(warp::reject::custom(ServiceError::ObjectNotFound))
}

async fn delete_question(id: String, store: Arc<RwLock<Store>>) -> Result<impl Reply, Rejection> {
    match store.write().questions.remove(&QuestId(id)) {
        Some(_) => Ok(warp::reply::with_status("", http::StatusCode::NO_CONTENT)),
        None => Err(warp::reject::custom(ServiceError::ObjectNotFound)),
    }
}

async fn get_question(id: String, store: Arc<RwLock<Store>>) -> Result<impl Reply, Rejection> {
    match store.read().questions.get(&QuestId(id)) {
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
    questions: HashMap<QuestId, Question>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Self::init(),
        }
    }

    fn init() -> HashMap<QuestId, Question> {
        let file = include_str!("../questions.json");
        if file.is_empty() {
            return HashMap::new();
        }
        serde_json::from_str(file).expect("Failed to read quetions from storage")
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

#[cfg(test)]
mod tests {
    use super::filters;
    use warp::test::request;
    use warp::http::StatusCode;

    #[tokio::test]
    async fn test_list_questions() {
        let api = filters::list_questions();
        let resp = request().method("GET").path("/questions").reply(&api).await;
        assert_eq!(resp.status(), StatusCode::ACCEPTED)
    }
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use warp::cors::CorsForbidden;
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

async fn list_guestions(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let res: Vec<Question> = store.questions.values().cloned().collect();
    Ok(warp::reply::json(&res))
}

async fn handle_err(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<CorsForbidden>() {
        return Ok(warp::reply::with_status(
            error.to_string(),
            http::StatusCode::FORBIDDEN,
        ));
    }

    Ok(warp::reply::with_status(
        "Route not found".to_string(),
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
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());
    let cors = warp::cors()
        .allow_methods(vec![http::Method::PUT, http::Method::DELETE])
        .allow_origins(vec!["http://front-end-service:3000"])
        .allow_header("content-type");

    let list_quest = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter)
        .and_then(list_guestions)
        .recover(handle_err);

    let routes = list_quest.with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 7878)).await;
}

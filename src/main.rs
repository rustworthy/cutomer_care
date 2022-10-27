use serde::Serialize;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use warp::cors::CorsForbidden;
use warp::{http, reject::Reject, Filter};
use warp::{Rejection, Reply};

#[derive(Debug, Serialize)]
struct QuestionId(String);

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
            false => Ok(QuestionId(id.to_string())),
        }
    }
}

#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

async fn list_guestions() -> Result<impl warp::Reply, warp::Rejection> {
    let q = Question::new(
        QuestionId::from_str("0").expect("No id provided"),
        "First Question Ever".to_string(),
        "Content of the first question ever".to_string(),
        Some(vec!["faq".to_string()]),
    );

    match q.id.0.is_empty() {
        true => Err(warp::reject::custom(InvalidId)),
        false => Ok(warp::reply::json(&q)),
    }
}

async fn handle_err(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<CorsForbidden>() {
        return Ok(warp::reply::with_status(
            error.to_string(),
            http::StatusCode::FORBIDDEN,
        ));
    }

    if let Some(InvalidId) = r.find() {
        return Ok(warp::reply::with_status(
            "No valid ID presented".to_string(),
            http::StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    Ok(warp::reply::with_status(
        "Route not found".to_string(),
        http::StatusCode::NOT_FOUND,
    ))
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_methods(vec![http::Method::PUT, http::Method::DELETE])
        .allow_origins(vec!["http://front-end-service:3000"])
        .allow_header("content-type");

    let list_quest = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(list_guestions)
        .recover(handle_err);

    let routes = list_quest.with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 7878)).await;
}

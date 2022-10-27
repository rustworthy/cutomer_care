use serde::Serialize;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use warp::Filter;

#[derive(Debug, Serialize)]
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

    Ok(warp::reply::json(&q))
}

#[tokio::main]
async fn main() {
    let list_quest = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(list_guestions);

    let routes = list_quest;

    warp::serve(routes).run(([127, 0, 0, 1], 7878)).await;
}

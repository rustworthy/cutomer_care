#![allow(dead_code)]

use warp::reject::Reject;
use warp::{Reply, Rejection};
use warp::cors::CorsForbidden;
use warp::body::BodyDeserializeError;
use warp::http::StatusCode;

#[derive(Debug)]
pub enum ServiceError {
    ParseError(std::num::ParseIntError),
    MissingParams,
    InvalidParamsRange,
    ObjectNotFound,
}

impl Reject for ServiceError {}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(err) => write!(f, "Failed to parse parameter: {}", err),
            Self::MissingParams => write!(f, "Missing parameter"),
            Self::InvalidParamsRange => write!(f, "Invalid parameters range"),
            Self::ObjectNotFound => write!(f, "Not found"),
        }
    }
}

pub async fn handle_err(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(err) = r.find::<CorsForbidden>() {
        return Ok(warp::reply::with_status(
            err.to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    if let Some(err) = r.find::<ServiceError>() {
        return Ok(warp::reply::with_status(
            err.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ));
    }

    if let Some(err) = r.find::<BodyDeserializeError>() {
        return Ok(warp::reply::with_status(
            err.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    };

    Ok(warp::reply::with_status(
        "Not found".to_string(),
        StatusCode::NOT_FOUND,
    ))
}
#![allow(dead_code)]

use warp::body::BodyDeserializeError;
use warp::cors::CorsForbidden;
use warp::http::StatusCode;
use warp::reject::Reject;
use warp::{Rejection, Reply};

#[derive(Debug)]
pub enum ServiceError {
    ParseError(std::num::ParseIntError),
    MissingParams,
    InvalidParamsRange,
    ObjectNotFound,
    DbQueryError,
    ExternalApiError
}

impl Reject for ServiceError {}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(err) => write!(f, "Failed to parse parameter: {}", err),
            Self::MissingParams => write!(f, "Missing parameter"),
            Self::InvalidParamsRange => write!(f, "Invalid parameters range"),
            Self::ObjectNotFound => write!(f, "Not found"),
            Self::DbQueryError => write!(f, "Query couldn't be executed"),
            Self::ExternalApiError => write!(f, "Error fetching data from external service"),
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

    if let Some(err) = r.find::<BodyDeserializeError>() {
        return Ok(warp::reply::with_status(
            err.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    };

    if let Some(ServiceError::DbQueryError) = r.find() {
        return Ok(warp::reply::with_status(
            ServiceError::DbQueryError.to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    if let Some(ServiceError::ObjectNotFound) = r.find() {
        return Ok(warp::reply::with_status(
            ServiceError::ObjectNotFound.to_string(),
            StatusCode::NOT_FOUND,
        ));
    }

    if let Some(err) = r.find::<ServiceError>() {
        return Ok(warp::reply::with_status(
            err.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    Ok(warp::reply::with_status(
        "Route not found".to_string(),
        StatusCode::NOT_FOUND,
    ))
}

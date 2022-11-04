#![allow(dead_code)]

use warp::reject::Reject;

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

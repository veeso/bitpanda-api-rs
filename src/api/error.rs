//! # Error

use reqwest::Error as HttpError;
use serde_json::Error as JsonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Http error: {0}")]
    Http(HttpError),
    #[error("No such asset: {0}")]
    NoSuchAsset(String),
    #[error("Parse error: {0}")]
    Parse(JsonError),
    #[error("Server error")]
    ServerError,
    #[error("Unexpected value: {0}")]
    UnexpectedValue(String),
    #[error("Client unauthorized")]
    Unauthorized,
}

impl From<HttpError> for ApiError {
    fn from(value: HttpError) -> Self {
        Self::Http(value)
    }
}

impl From<JsonError> for ApiError {
    fn from(value: JsonError) -> Self {
        Self::Parse(value)
    }
}

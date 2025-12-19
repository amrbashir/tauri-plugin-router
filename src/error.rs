use serde::Serialize;
use tauri::http::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum Error {
    #[error("command not found: {0}")]
    CommandNotFound(String),

    #[error("invalid arguments: {0}")]
    InvalidArgs(String),

    #[error("deserialization error: {0}")]
    DeserializationError(String),

    #[error("serialization error: {0}")]
    SerializationError(String),

    #[error("webview not found: {0}")]
    WebviewNotFound(String),
}

impl Error {
    /// Returns the appropriate HTTP status code for the error.
    pub fn status_code(&self) -> StatusCode {
        match self {
            Error::CommandNotFound(_) => StatusCode::NOT_FOUND,
            Error::InvalidArgs(_) => StatusCode::BAD_REQUEST,
            Error::DeserializationError(_) => StatusCode::BAD_REQUEST,
            Error::SerializationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::WebviewNotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}

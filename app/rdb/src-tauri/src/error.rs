use std::sync::PoisonError;

use serde::Serialize;

/// Interop with pure/result in JS side
#[derive(Debug, Clone, Serialize)]
pub struct ResultInterop<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err: Option<Error>,
}

impl<T: Serialize> ResultInterop<T> {
    pub fn ok(val: T) -> Self {
        Self {
            val: Some(val),
            err: None,
        }
    }
    pub fn err<S: Into<Error>>(err: S) -> Self {
        Self {
            val: None,
            err: Some(err.into()),
        }
    }
}

impl<T: Serialize, E: Into<Error>> From<Result<T, E>> for ResultInterop<T> {
    fn from(r: Result<T, E>) -> Self {
        match r {
            Ok(val) => Self {
                val: Some(val),
                err: None,
            },
            Err(err) => Self {
                val: None,
                err: Some(err.into()),
            },
        }
    }
}

#[derive(Debug, Clone, thiserror::Error, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    #[error("io error: {0}")]
    IOError(String),
    // #[error("lock was poisoned: {0}")]
    // PoisonError(String),
    // #[error("there are too many tasks pending, probably a leak")]
    // ExecutorIdUnavailable,
    #[error("executor error: {0}")]
    ExecutorError(#[from] rdata::executor::Error),
    #[error("database error: {0}")]
    DatabaseError(#[from] rdata::db::Error),
    #[error("no search result found. Please search first")]
    MissingSearchResult,
    #[error("invalid data detected while reading search result. Please search again.")]
    InvalidSearchResult,
    #[error("{0}")]
    Generic(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e.to_string())
    }
}

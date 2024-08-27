use std::sync::PoisonError;

use serde::Serialize;
use serde_json::error;

/// Interop with pure/result in JS side
#[derive(Debug, Clone, Serialize)]
pub struct ResultInterop<T> {
    pub val: Option<T>,
    pub err: Option<String>,
}

impl<T: Serialize> ResultInterop<T> {
    pub fn ok(val: T) -> Self {
        Self {
            val: Some(val),
            err: None,
        }
    }
    pub fn err<S: Into<String>>(err: S) -> Self {
        Self {
            val: None,
            err: Some(err.into()),
        }
    }
}

impl<T: Serialize> From<Result<T, Error>> for ResultInterop<T> {
    fn from(r: Result<T, Error>) -> Self {
        match r {
            Ok(val) => Self {
                val: Some(val),
                err: None,
            },
            Err(err) => Self {
                val: None,
                err: Some(err.to_string()),
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("lock was poisoned: {0}")]
    PoisonError(String),
    #[error("there are too many tasks pending, probably a leak")]
    ExecutorIdUnavailable,
    #[error("database error: {0}")]
    DatabaseError(#[from] rdata::db::Error),
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("no search result found. Please search first")]
    MissingSearchResult,
    #[error("invalid data detected while reading search result. Please search again.")]
    InvalidSearchResult,
    #[error("{0}")]
    Generic(String),
}

impl<T> From<PoisonError<T>> for Error {
    fn from(e: PoisonError<T>) -> Self {
        Error::PoisonError(e.to_string())
    }
}

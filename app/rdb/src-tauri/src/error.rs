use serde::Serialize;
use ts_rs::TS;

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

#[derive(Debug, Clone, thiserror::Error, Serialize, TS)]
#[ts(export)]
#[serde(tag = "type", content = "data")]
#[allow(clippy::enum_variant_names)]//, reason="readability from TypeScript")]
pub enum Error {
    #[error("io error: {0}")]
    IOError(String),
    #[error("lock was poisoned: {0}")]
    PoisonError(String),
    #[error("there are too many tasks pending, probably a leak")]
    ExecutorUnavailable,
    #[error("aborted")]
    Aborted,

    #[error("database error: {0}")]
    DatabaseError(#[from] rdata::db::Error),
    #[error("no search result found. Please search first")]
    MissingSearchResult,
    #[error("{0}")]
    Generic(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e.to_string())
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(e: std::sync::PoisonError<T>) -> Self {
        Error::PoisonError(e.to_string())
    }
}

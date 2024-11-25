use serde::Serialize;
use ts_rs::TS;

/// Database-related errors
#[derive(Debug, Clone, thiserror::Error, Serialize, TS)]
#[ts(export, rename = "DatabaseError")]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // General errors
    #[error("io error accessing database: {0}")]
    IO(String),
    #[error("YAML error: {0}")]
    YAML(String),
    #[error("database query failed when cooking: {0}")]
    Cooking(#[from] crate::cook::Error),

    // Database errors
    #[error("database is locked by another instance of the app")]
    Locked,
    #[error("cannot find index.yaml")]
    MissingIndex,
    #[error("invalid chunk count: expected {0}, got {1}")]
    InvalidIndexChunkCount(usize, usize),
    #[error("cannot find chunk_{0}.rdb")]
    MissingChunk(usize),
    #[error("invalid chunk size: expected {0} bytes, got {1} bytes")]
    InvalidChunkSize(usize, usize),
    #[error("invalid recipe id from temporary result: {0}")]
    InvalidRecipeId(usize),
    #[error("there are too many temporary results. Try closing the app and restart it")]
    TooManyTemporary,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e.to_string())
    }
}

impl From<serde_yaml_ng::Error> for Error {
    fn from(e: serde_yaml_ng::Error) -> Self {
        Error::YAML(e.to_string())
    }
}

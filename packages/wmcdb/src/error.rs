use serde::Serialize;

/// Database-related errors
#[derive(Debug, Clone, thiserror::Error, Serialize)]
#[serde(tag = "type", content = "data")]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export, rename = "DatabaseError"))]
pub enum Error {
    // General errors
    #[error("io error accessing database: {0}")]
    IO(String),
    #[error("YAML error: {0}")]
    YAML(String),

    // Database errors
    #[error("database is locked by another instance of the app")]
    Locked,
    #[error("cannot find index.yaml")]
    MissingIndex,
    #[error("invalid chunk count: expected {0}, got {1}")]
    InvalidIndexChunkCount(u32, u32),
    #[error("cannot find chunk {0}")]
    MissingChunk(u32),
    #[error("invalid chunk size: expected {0} bytes, got {1} bytes")]
    InvalidChunkSize(usize, usize),
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

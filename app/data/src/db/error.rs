
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error accessing database: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Invalid chunk size: {0} (expected {1})")]
    InvalidChunkSize(usize, usize),
}

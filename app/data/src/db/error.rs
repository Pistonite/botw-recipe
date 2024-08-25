#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error accessing database: {0}")]
    IOError(#[from] std::io::Error),
    #[error("YAML error: {0}")]
    YAMLError(#[from] serde_yaml::Error),
    #[error("Invalid database: {0}")]
    InvalidDatabase(String),
    #[error("Cooking error: {0}")]
    Cooking(#[from] crate::cook::Error),
}

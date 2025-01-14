use serde::Serialize;
use ts_rs::TS;

/// Cooking-related errors
#[derive(Debug, Clone, thiserror::Error, Serialize, TS)]
#[ts(export, rename = "CookError")]
#[serde(tag = "type", content = "data")]
pub enum Error {
    #[error("YAML error: {0}")]
    Yaml(String),
    #[error("failed to read ingredients: {0:?}")]
    ReadIngr(Vec<Error>),
    #[error("failed to read recipes: {0}")]
    ReadRecipe(String),
    #[error("cannot find ingredient: {0}.")]
    ItemNotFound(String),
    #[error("ambiguous ingredient: {0}, which can be: {1:?}")]
    AmbiguousIngr(String, Vec<String>),
    #[error("too many ingredients! At most 5 are allowed.")]
    TooManyIngr,
    #[error("not enough ingredients! At least 1 is required.")]
    TooFewIngr,
    #[error("invalid recipe id: {0}")]
    InvalidRecipeId(u64),
    #[error("unexpected data error: {0}")]
    Data(String),
}

impl From<serde_yaml_ng::Error> for Error {
    fn from(e: serde_yaml_ng::Error) -> Self {
        Self::Yaml(e.to_string())
    }
}

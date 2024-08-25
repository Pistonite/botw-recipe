use crate::Actor;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("failed to read ingredients for: {0:?}")]
    ReadIngr(Vec<Error>),
    #[error("failed to read recipes: {0}")]
    ReadRecipe(String),
    #[error("attempting to get data for CookEffect::None")]
    NoEffectData,
    #[error("cannot find ingredient: {0}.")]
    ItemNotFound(String),
    #[error("ambiguous ingredient: {0}, which can be: {1:?}")]
    AmbiguousIngr(String, Vec<Actor>),
    #[error("too many ingredients! At most 5 are allowed.")]
    TooManyIngr,
    #[error("not enough ingredients! At least 1 is required.")]
    TooFewIngr,
    #[error("invalid recipe id: {0}")]
    InvalidRecipeId(usize),
    #[error("unexpected data error: {0}")]
    Data(String),
}

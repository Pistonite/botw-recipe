use botw_recipe::cook::{CookData, CookDataInvalidReason};

/// Enum for the data utils
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("!! io error reading chunk")]
    IO(#[from] std::io::Error),
    #[error("!! invalid size: actual {0} != expected {1}")]
    InvalidSize(usize, usize),
    #[error("!! invalid record at {0}: {1:?}; data={2:?}")]
    InvalidRecord(usize, CookDataInvalidReason, CookData),
    #[error("!! first mismatch at {0}: {1:?} != {2:?}; matched {3}")]
    Mismatch(usize, CookData, CookData, usize /*matched_count*/),
    #[error("!! chunk not found")]
    NotFound,
}

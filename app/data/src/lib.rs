mod generated;
pub use generated::constants::*;
pub use generated::{get_compact_chunk_record_size, get_raw_chunk_record_size};
pub use generated::{Actor, Group};

pub mod cook;
mod recipe;
pub use recipe::{RecipeId, RecipeInputs};
pub mod db;
pub mod wmc;

/// Get the number of ways to choose `k` items from `n` items, allowing for repetition
///
/// The time complexity is O(1) because all values are pre-computed.
pub fn multichoose(n: usize, k: usize) -> usize {
    MULTICHOOSE[n][k]
}

macro_rules! debugln {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug-print")]
        println!($($arg)*);
    }
}
pub(crate) use debugln;

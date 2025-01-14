mod generated;
// pub use generated::constants::*;
// pub use generated::{Group};

pub mod cook;
// mod recipe;
// pub use recipe::{RecipeId, RecipeInputs};
pub mod db;
pub mod wmc;

/// The file system database
pub mod fsdb;
// pub use fsdb::convert::{RecipeId, RecipeInputs};

// /// Get the number of ways to choose `k` items from `n` items, allowing for repetition
// ///
// /// The time complexity is O(1) because all values are pre-computed.
// pub fn multichoose(n: usize, k: usize) -> usize {
//     MULTICHOOSE[n][k]
// }

macro_rules! debugln {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug-print")]
        println!($($arg)*);
    }
}
pub(crate) use debugln;

// mod distr;


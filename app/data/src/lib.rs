mod generated;
pub use generated::constants::*;
pub use generated::{Actor, Group};

pub mod recipe;
pub mod cook;
pub mod wmc;
//
pub mod db;

use serde::{Deserialize, Serialize};

use recipe::RecipeInputs;
use cook::CookData;

/// Get the number of ways to choose `k` items from `n` items, allowing for repetition
///
/// The time complexity is O(1) because all values are pre-computed.
pub fn multichoose(n: usize, k: usize) -> usize {
    MULTICHOOSE[n][k]
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    pub data: CookData,
    pub inputs: Vec<String>,
}

impl Recipe {
    pub fn new(data: CookData, inputs: RecipeInputs) -> Self {
        Self {
            data,
            inputs: inputs.to_names().into_iter().map(|a| a.to_string()).collect(),
        }
    }
}

// use std::fs;
// use bit_set::BitSet;


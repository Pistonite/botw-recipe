use rdata::recipe::RecipeId;
use serde::Serialize;
use tauri::{AppHandle, Manager};

use crate::error::ResultInterop;

/// Database is initialized and ready to use
pub fn emit_initialized(app: &AppHandle) {
    let _ = app.emit_all("initialized", ());
}

/// Search for recipes has been completed
pub fn emit_search_complete(app: &AppHandle, result: ResultInterop<SearchComplete>) {
    let _ = app.emit_all("search-complete", result);
}
/// Data for the `search-complete` event
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchComplete {
    /// Number of recipes found
    pub found_count: usize,
    /// Actors in those recipes
    ///
    /// Position corresponds to actor id, value to the number of recipes
    pub actors: Vec<usize>,
}

/// Send search progress as a percentage between 0 and 100
pub fn emit_search_progress(app: &AppHandle, percentage: u32) {
    let _ = app.emit_all("search-progress", percentage);
}

pub fn emit_filter_complete(app: &AppHandle, result: ResultInterop<FilterComplete>) {
    let _ = app.emit_all("filter-complete", result);
}

#[derive(Debug, Clone, Serialize)]
pub struct FilterComplete {
    pub results: Vec<RecipeInfo>,
}
#[derive(Debug, Clone, Serialize)]
pub struct RecipeInfo {
    #[serde(skip)]
    pub recipe_id: RecipeId,
    pub groups: [usize; rdata::NUM_INGR],
    pub value: i32,
    pub price: i32,
}

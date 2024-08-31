use enum_map::EnumMap;
use rdata::Actor;
use serde::Serialize;
use tauri::{AppHandle, Manager};
use ts_rs::TS;

use crate::error::{Error, ResultInterop};

/// Database is initialized and ready to use
pub fn emit_initialized(app: &AppHandle) {
    let _ = app.emit_all("initialized", ());
}

/// Search for recipes has been completed
pub fn emit_search_complete(app: &AppHandle, count: usize, actors: &EnumMap<Actor, usize>) {
    let actors = actors.into_values().collect();
    let _ = app.emit_all(
        "search-complete",
        ResultInterop::ok(SearchComplete {
            found_count: count,
            actors: Some(actors),
        }),
    );
}
/// Search for recipes encounted an error
pub fn emit_search_complete_err<E: Into<Error>>(app: &AppHandle, err: E) {
    let _ = app.emit_all(
        "search-complete",
        ResultInterop::<SearchComplete>::err(err.into()),
    );
}
/// Search for recipes completed, but actor meta not processed
pub fn emit_search_complete_no_actors(app: &AppHandle, count: usize) {
    let _ = app.emit_all(
        "search-complete",
        ResultInterop::ok(SearchComplete {
            found_count: count,
            actors: None,
        }),
    );
}
/// Data for the `search-complete` event
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
struct SearchComplete {
    /// Number of recipes found
    pub found_count: usize,
    /// Actors in those recipes
    ///
    /// Position corresponds to actor id, value to the number of recipes.
    /// Search may not provide actor details if there are too many results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actors: Option<Vec<usize>>,
}

/// Send search progress as a percentage between 0 and 100
pub fn emit_search_progress(app: &AppHandle, percentage: u32) {
    let _ = app.emit_all("search-progress", percentage);
}

// pub fn emit_filter_complete(app: &AppHandle, result: ResultInterop<FilterComplete>) {
//     let _ = app.emit_all("filter-complete", result);
// }
//
// #[derive(Debug, Clone, Serialize)]
// pub struct FilterComplete {
//     pub results: Vec<RecipeInfo>,
// }
// #[derive(Debug, Clone, Serialize)]
// pub struct RecipeInfo {
//     #[serde(skip)]
//     pub recipe_id: RecipeId,
//     pub groups: [usize; rdata::NUM_INGR],
//     pub value: i32,
//     pub price: i32,
// }

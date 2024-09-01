use enum_map::EnumMap;
use rdata::Group;
use serde::Serialize;
use tauri::{AppHandle, Manager};
use ts_rs::TS;

use crate::error::{Error, ResultInterop};

/// Database is initialized and ready to use
pub fn emit_initialized(app: &AppHandle) {
    let _ = app.emit_all("initialized", ());
}

/// Search for recipes has been completed
pub fn emit_search_complete(app: &AppHandle, count: usize, groups: &EnumMap<Group, usize>) {
    let groups = groups.into_values().collect();
    let _ = app.emit_all(
        "search-complete",
        ResultInterop::ok(Stats {
            found_count: count,
            group_stat: Some(groups),
        }),
    );
}
/// Filter for recipes has been completed
pub fn emit_filter_complete(app: &AppHandle, count: usize, groups: &EnumMap<Group, usize>) {
    let groups = groups.into_values().collect();
    let _ = app.emit_all(
        "filter-complete",
        ResultInterop::ok(Stats {
            found_count: count,
            group_stat: Some(groups),
        }),
    );
}
/// Search for recipes encounted an error
pub fn emit_search_complete_err<E: Into<Error>>(app: &AppHandle, err: E) {
    let _ = app.emit_all("search-complete", ResultInterop::<Stats>::err(err.into()));
}
/// Filter for recipes encounted an error
pub fn emit_filter_complete_err<E: Into<Error>>(app: &AppHandle, err: E) {
    let _ = app.emit_all("filter-complete", ResultInterop::<Stats>::err(err.into()));
}
/// Search for recipes completed, but group stat not processed
pub fn emit_search_complete_no_stat(app: &AppHandle, count: usize) {
    let _ = app.emit_all(
        "search-complete",
        ResultInterop::ok(Stats {
            found_count: count,
            group_stat: None,
        }),
    );
}
/// Stats for searching and filtering
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
struct Stats {
    /// Number of recipes found
    pub found_count: usize,
    /// Groups in those recipes
    ///
    /// Position corresponds to group id, value to the number of recipes.
    /// Search may not provide actor details if there are too many results
    pub group_stat: Option<Vec<usize>>,
}

/// Send search progress as a percentage between 0 and 100
pub fn emit_search_progress(app: &AppHandle, percentage: u32) {
    let _ = app.emit_all("search-progress", percentage);
}

/// Send filter progress as a percentage between 0 and 100
pub fn emit_filter_progress(app: &AppHandle, percentage: u32) {
    let _ = app.emit_all("filter-progress", percentage);
}
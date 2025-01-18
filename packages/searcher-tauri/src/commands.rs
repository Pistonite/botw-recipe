use std::sync::{Arc, LazyLock};

use botw_recipe_sys::Group;
use botw_recipe_wmcdb::Filter;
use enumset::EnumSet;
use log::{info, error};
use tauri::{AppHandle, Manager, State};

use crate::{events, stages, Global, ResultInterop};

/// Set the title of the native application. 
/// Also show the window if not already shown
#[tauri::command]
pub fn set_title(title: String, app: AppHandle) {
    let Some(window) = app.get_window("main") else {
        error!("main window not found!!");
        return;
    };
    // showing window here because at this point, JS side has
    // definitely finished initial rendering, so we don't
    // get a white window in dark mode
    info!("showing window");
    if let Err(e) = window.show() {
        error!("fail to show window: {}", e);
    }
    info!("setting window title to {}", title);
    if let Err(e) = window.set_title(&title) {
        error!("fail to set window title: {}", e);
    }
}

/// Run initialization in worker threads.
///
/// JS side should call this after UI load, and prevent calling other commands
/// until the `initialized` event is received. Otherwise, accessing DB could
/// block the main thread.
#[tauri::command]
pub fn initialize(app: AppHandle, state: State<Global>) {
    info!("initializing state");
    let db = Arc::clone(&state.db);
    state.executor.pool().execute(move || {
        LazyLock::force(&db);
        events::emit_initialized(&app);
    });
}

/// Starts a DB scan with the given filter.
/// Returns a list of handles to abort the search by calling the abort command
/// for each handle.
///
/// The search is optimized by skipping chunks that are filtered by index.
/// The result is emitted through the `search-complete` event.
///
/// JS side should make sure to abort previous searches before starting a new one.
///
/// One or more `search-progress` event might be emitted during the search.
/// The payload is a number between 0 and 100 indicating the progress percentage.
#[tauri::command]
pub fn search(filter: Filter, app: AppHandle, state: State<Global>) -> ResultInterop<()> {
    stages::search::run(&filter, app, state).into()
}

/// Abort the active search task
#[tauri::command]
pub fn abort_search(state: State<Global>) -> ResultInterop<()> {
    stages::search::abort(state).into()
}

/// Execute filtering on the search result based on what actors should be included.
/// The filter results are returned through the `filter-complete` event.
///
/// JS side should make sure to abort previous filter before starting a new one.
///
/// One or more `filter-progress` event might be emitted during the filter.
/// The payload is a number between 0 and 100 indicating the progress percentage.
#[tauri::command]
pub fn filter(include: Vec<u8>, app: AppHandle, state: State<Global>) -> ResultInterop<()> {
    let mut filter = include
        .into_iter()
        .filter_map(Group::from_u8)
        .collect::<EnumSet<_>>();
    // always include "none" i.e. the empty space
    filter.insert(Group::None);
    stages::filter::run(&Arc::new(filter), app, state).into()
}

/// Abort the active filter task
#[tauri::command]
pub fn abort_filter(state: State<Global>) -> ResultInterop<()> {
    stages::filter::abort(state).into()
}

/// Cook the filter result into a list of recipes.
#[tauri::command]
pub fn cook(app: AppHandle, state: State<Global>) -> ResultInterop<()> {
    stages::cook::run(app, state).into()
}

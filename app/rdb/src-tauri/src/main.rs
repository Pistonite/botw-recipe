// Prevents additional console window on Windows in release
#![cfg_attr(not(feature = "devtools"), windows_subsystem = "windows")]

use std::collections::HashSet;
use std::ops::Deref;
use std::sync::{Arc, LazyLock, Mutex};

use enum_map::Enum;
use log::info;
use rdata::db::{Database, Filter, TempResult};
use rdata::Group;
use tauri::{AppHandle, Manager, RunEvent, State, WindowEvent};

mod error;
mod events;
mod executor;
use executor::Executor;
mod file_io;
mod filter;
mod search;
mod tasks;

use error::{Error, ResultInterop};

/// Tauri app global state
pub struct Global {
    /// The task executor
    pub executor: Arc<Executor>,
    /// The database handle
    pub db: Arc<LazyLock<Result<Database, Error>>>,
    /// Handle for the result of the last search
    pub search_result: Arc<Mutex<Option<TempResult>>>,
    /// Abort handles for the current search
    pub search_handles: Arc<Mutex<Vec<usize>>>,

    pub filter_result: Arc<Mutex<Option<TempResult>>>,
    pub last_included: Arc<Mutex<HashSet<Group>>>,
    /// Abort handles for the current filter
    pub filter_handles: Arc<Mutex<Vec<usize>>>,
    // /// Filter sub-state
    // filter: Arc<RwLock<FilterState>>,
}

impl Global {
    pub fn get_db(&self) -> Result<&Database, Error> {
        let db = self.db.as_ref().deref();
        match db {
            Ok(db) => Ok(db),
            Err(e) => Err(e.clone()),
        }
    }
}

////////////////////////////////// Commands //////////////////////////////////
#[tauri::command]
fn set_title(title: String, app: AppHandle) {
    if let Some(window) = app.get_window("main") {
        info!("setting window title to {}", title);
        let _ = window.set_title(&title);
    }
}

/// Run initialization in worker threads.
///
/// JS side should call this after UI load, and prevent calling other commands
/// until the `initialized` event is received. Otherwise, accessing DB could
/// block the main thread.
#[tauri::command]
fn initialize(app: AppHandle, state: State<Global>) {
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
fn search(filter: Filter, app: AppHandle, state: State<Global>) -> ResultInterop<()> {
    search::run(&filter, app, state).into()
}

#[tauri::command]
fn abort_search(state: State<Global>) -> ResultInterop<()> {
    search::abort(state).into()
}
/// Execute filtering on the search result based on what actors should be included.
/// The filter results are returned through the `filter-complete` event.
///
/// JS side should make sure to abort previous filter before starting a new one.
///
/// One or more `filter-progress` event might be emitted during the filter.
/// The payload is a number between 0 and 100 indicating the progress percentage.
#[tauri::command]
fn filter(include: Vec<usize>, app: AppHandle, state: State<Global>) -> ResultInterop<()> {
    let mut filter = include
        .into_iter()
        .map(Group::from_usize)
        .collect::<HashSet<_>>();
    // always include "none" i.e. the empty space
    filter.insert(Group::None);
    filter::run(&Arc::new(filter), app, state).into()
}

#[tauri::command]
fn abort_filter(state: State<Global>) -> ResultInterop<()> {
    filter::abort(state).into()
}

fn main() {
    env_logger::init();
    info!("configuring application");
    let executor = Arc::new(Executor::new(num_cpus::get()));
    let db = Arc::new(file_io::create_database());

    let app = tauri::Builder::default()
        .manage(Global {
            executor: Arc::clone(&executor),
            db: Arc::clone(&db),
            search_result: Arc::new(Mutex::new(None)),
            search_handles: Arc::new(Mutex::new(Vec::new())),
            filter_result: Arc::new(Mutex::new(None)),
            filter_handles: Arc::new(Mutex::new(Vec::new())),
            last_included: Arc::new(Mutex::new(HashSet::new())),
            // filter: Arc::new(RwLock::new(FilterState::default())),
        })
        .invoke_handler(tauri::generate_handler![
            set_title,
            initialize,
            search,
            abort_search,
            filter,
            abort_filter
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    info!("starting application");

    app.run(move |app_handle, e| match e {
        RunEvent::WindowEvent {
            label,
            event: WindowEvent::CloseRequested { .. },
            ..
        } => {
            if label != "main" {
                return;
            }
            info!("closing application");
            let app = app_handle.clone();
            executor.pool().execute(move || {
                if let Some(window) = app.get_window("main") {
                    let _ = window.close();
                }
                info!("window closed");
            });
            let db = Arc::clone(&db);
            executor.pool().execute(move || {
                // tauri doesn't drop its state
                if let Ok(db) = db.as_ref().deref() {
                    db.close();
                }
                info!("database closed");
            });
        }
        RunEvent::ExitRequested { .. } => {
            info!("waiting for executor to finish");
            executor.join();
            info!("exiting application");
        }
        _ => {}
    });
}

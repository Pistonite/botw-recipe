// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(feature = "devtools"), windows_subsystem = "windows")]

use std::cell::{OnceCell, UnsafeCell};
use std::collections::HashSet;
use std::ops::Deref;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::mpsc;
use std::sync::{Arc, LazyLock, OnceLock, RwLock};

use enum_map::{Enum, EnumMap};
use itertools::Itertools;
use log::{error, info, debug};
use rdata::db::{Database, Filter, TempResult};
use rdata::recipe::{RecipeId, RecipeInputs};
use rdata::wmc::WeaponModifierSet;
use rdata::{Actor, Group, Executor};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, RunEvent, State, WindowEvent};

mod error;
mod events;
mod file_io;
mod search;

use error::{Error, ResultInterop};

/// Tauri app global state
pub struct Global {
    /// The task executor
    pub executor: Arc<Executor>,
    /// The database handle
    pub db: Arc<LazyLock<Result<Database, Error>>>,
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


#[derive(Default)]
struct FilterState {
    search_result: Option<TempResult>,
    // filter_result: Option<TempResult>,
    // /Last filter used for the search, used to optimize the next filter
    // last_filter: HashSet<Actor>,
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

/// Abort a task by its handle
#[tauri::command]
fn abort(handle: usize, state: State<Global>) -> ResultInterop<()> {
    state.executor.abort(handle).into()
}

/// Starts a DB linear search with the given filter.
/// Returns a list of handles to abort the search by calling the abort command
/// for each handle.
///
/// The search is optimized by skipping chunks that are filtered by index.
/// The result is emitted through the `search-complete` event. If the event
/// returns an error, JS side owns aborting the search. JS side should also
/// make sure to abort previous searches before starting a new one.
///
/// One or more `search-progress` event might be emitted during the search.
/// The payload is a number between 0 and 100 indicating the progress percentage.
#[tauri::command]
fn search(filter: Filter, app: AppHandle, state: State<Global>) -> ResultInterop<Vec<usize>> {
    search::run(&filter, app, state).into()
}


// /// Execute filtering on the search result based on what actors should be included.
// /// The filter results are returned through the `filter-complete` event.
// ///
// /// JS side must ensure that only one filtering is being executed at a time (i.e.
// /// `filter_actors` should not be called before `filter-complete` event is received)
// #[tauri::command]
// fn filter_actors(filter: Vec<usize>, app: AppHandle, state: State<Global>) -> ResultInterop<()> {
//     let mut filter = filter
//         .into_iter()
//         .map(Actor::from_usize)
//         .collect::<HashSet<_>>();
//     // always include "none" i.e. the empty space
//     filter.insert(Actor::None);
//     filter_actors_impl(&Arc::new(filter), app, state).into()
// }
//
// fn filter_actors_impl(
//     filter: &Arc<HashSet<Actor>>,
//     app: AppHandle,
//     state: State<Global>,
// ) -> Result<(), Error> {
//     info!("filtering actors: {:?}", filter);
//     let mut filter_state = state.filter.write()?;
//     // if the new filter is a subset of the last filter, we can reuse the results
//     if filter_state.is_result_loaded
//         && filter
//             .intersection(&filter_state.last_filter)
//             .copied()
//             .collect::<HashSet<_>>()
//             == **filter
//     {
//         info!("reusing filtered results from last filtering");
//         // use the filtered results
//         let results = std::mem::take(&mut filter_state.filtered_results);
//         filter_actors_with_iter(results, filter, app, &state)?;
//     } else {
//         info!("filtering from scratch from search result");
//         // load the filtered results from saved search result
//         let reader = file_io::open_search_result()?;
//         filter_actors_with_iter(reader.map_while(|x| x.ok()), filter, app, &state)?;
//     }
//     Ok(())
// }
//
// fn filter_actors_with_iter<I: IntoIterator<Item = RecipeId>>(
//     iter: I,
//     filter: &Arc<HashSet<Actor>>,
//     app: AppHandle,
//     state: &State<Global>,
// ) -> Result<(), Error> {
//     // chunking the recipes to 4096 to reduce scheduling overhead
//     const CHUNK_SIZE: usize = 4096;
//     let (send, recv) = mpsc::channel::<Result<Vec<events::RecipeInfo>, Error>>();
//     {
//         let filter_state = Arc::clone(&state.filter);
//         let filter = Arc::clone(filter);
//         state.executor.background_pool().execute(move || {
//             let mut recipes = Vec::new();
//             for result in recv {
//                 match result {
//                     Ok(filtered) => {
//                         recipes.extend(filtered);
//                     }
//                     Err(err) => {
//                         events::emit_filter_complete(&app, ResultInterop::err(err));
//                         return;
//                     }
//                 }
//             }
//             // update filter_state on completion
//             {
//                 if let Ok(mut filter_state) = filter_state.write() {
//                     filter_state.is_result_loaded = true;
//                     filter_state.filtered_results.clear();
//                     filter_state
//                         .filtered_results
//                         .extend(recipes.iter().map(|info| info.recipe_id));
//                     filter_state.last_filter.clear();
//                     filter_state.last_filter.extend(filter.iter().copied());
//                 }
//             }
//             info!("filtering complete, {} recipes found", recipes.len());
//             events::emit_filter_complete(&app, ResultInterop::ok(events::FilterComplete { results: recipes }));
//         });
//     }
//     for chunk in &iter.into_iter().chunks(CHUNK_SIZE) {
//         let chunk: Vec<RecipeId> = chunk.collect();
//         let filter = Arc::clone(filter);
//         let send = send.clone();
//         let pot = get_db(state)?.pot();
//         state.executor.pool().execute(move || {
//             let mut filtered = Vec::with_capacity(CHUNK_SIZE);
//             for recipe_id in chunk {
//                 // add the recipe if any actor in the recipe is in the filter
//                 let inputs: RecipeInputs = recipe_id.into();
//                 for group in inputs.as_slice() {
//                     if group.actors().iter().any(|actor| filter.contains(actor)) {
//                         // cook me
//                         let result = match pot.cook_inputs(recipe_id) {
//                             Ok(result) => result,
//                             Err(err) => {
//                                 let _ = send.send(Err(err.to_string()));
//                                 return;
//                             }
//                         };
//
//                         let mut group_ids = [0usize; rdata::NUM_INGR];
//                         for (i, group) in inputs.as_slice().iter().enumerate() {
//                             group_ids[i] = *group as usize;
//                         }
//
//                         let info = RecipeInfo {
//                             recipe_id,
//                             groups: group_ids,
//                             value: result.data.health_recover,
//                             price: result.data.sell_price,
//                         };
//
//                         filtered.push(info);
//                         break;
//                     }
//                 }
//             }
//             let _ = send.send(Ok(filtered));
//         });
//     }
//     drop(send);
//     Ok(())
// }

fn main() {
    env_logger::init();
    info!("starting application");

    let executor = Arc::new(Executor::new(num_cpus::get()));
    let db = Arc::new(file_io::create_database());

    let app = tauri::Builder::default()
        .manage(Global {
            executor: Arc::clone(&executor),
            db: Arc::clone(&db),
            // filter: Arc::new(RwLock::new(FilterState::default())),
        })
        .invoke_handler(tauri::generate_handler![
            set_title,
            initialize,
            abort,
            search,
            // filter_actors
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

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
        RunEvent::ExitRequested { ..}=> {
            info!("waiting for executor to finish");
            executor.join();
            info!("exiting application");
        }
        _ => {}
    });
}

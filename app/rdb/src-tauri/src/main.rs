// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::cell::{OnceCell, UnsafeCell};
use std::collections::HashSet;
use std::ops::Deref;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::{Arc, LazyLock, OnceLock, RwLock};
use std::sync::mpsc;

use enum_map::{Enum, EnumMap};
use file_io::save_search_result;
use itertools::Itertools;
use log::{info, error};
use rdata::recipe::{RecipeId, RecipeInputs};
use rdata::wmc::WeaponModifierSet;
use rdata::{Actor, Group};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};
use rdata::db::{Database, Filter};

mod executor;
mod error;
mod file_io;

use executor::Executor;
use error::{Error, ResultInterop};



/// Tauri app global state
struct Global {
    /// The task executor
    executor: Executor,
    /// The database handle
    db: Arc<LazyLock<Result<Database, Error>>>,
    /// Filter sub-state
    filter: Arc<RwLock<FilterState>>,
}

fn get_db<'a>(state: &'a State<Global>) -> Result<&'a Database, Error> {
    let db = state.db.as_ref().deref();
    match db {
        Ok(db) => Ok(db),
        Err(e) => Err(Error::Generic(e.to_string())),
    }
}

#[derive(Default)]
struct FilterState {
    /// Are `filtered_results` loaded?
    is_result_loaded: bool,
    /// Filtered results stored in memory
    filtered_results: Vec<RecipeId>,
    /// Last filter used for the search, used to optimize the next filter
    last_filter: HashSet<Actor>,
}

////////////////////////////////// Events //////////////////////////////////

fn emit_initialized(app: &AppHandle) {
    let _ = app.emit_all("initialized", ());
}

/// Data for the `search-complete` event
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchComplete {
    /// Number of recipes found
    result_count: usize,
    /// Actors in those recipes
    /// 
    /// Position corresponds to actor id, value to the number of recipes
    actors: Vec<usize>,
}
fn emit_search_complete(app: &AppHandle, result: ResultInterop<SearchComplete>) {
    let _ = app.emit_all("search-complete", result);
}

#[derive(Debug, Clone, Serialize)]
struct FilterComplete {
    results: Vec<RecipeInfo>,
}
#[derive(Debug, Clone, Serialize)]
struct RecipeInfo {
    #[serde(skip)]
    recipe_id: RecipeId,
    groups: [usize; rdata::NUM_INGR],
    value: i32,
    price: i32,
}
fn emit_filter_complete(app: &AppHandle, result: ResultInterop<FilterComplete>) {
    let _ = app.emit_all("filter-complete", result);
}

////////////////////////////////// Commands //////////////////////////////////

#[derive(Debug, Clone, Deserialize)]
struct InitArg {
    /// Localized window title
    title: String
}
/// Run initialization in worker threads.
/// 
/// JS side should call this after UI load, and prevent calling other commands
/// until the `initialized` event is received. Otherwise, accessing DB could
/// block the main thread.
#[tauri::command]
fn initialize(arg: InitArg, app: AppHandle, state: State<Global>) {
    info!("initializing state");
    let db = Arc::clone(&state.db);
    if let Some(window) = app.get_window("main") {
        info!("setting window title to {}", arg.title);
        let _ = window.set_title(&arg.title);
    }
    state.executor.pool().execute(move || {
        LazyLock::force(&db);
        emit_initialized(&app);
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
#[tauri::command]
fn search(filter: Filter, app: AppHandle, state: State<Global>) -> ResultInterop<Vec<usize>> {
    search_impl(&filter, app, state).into()
}

fn search_impl(filter: &Filter, app: AppHandle, state: State<Global>) -> Result<Vec<usize>, Error> {
    info!("searching with filter: {:?}", filter);
    state.executor.clear_abort_handles()?;
    let db: &Database = get_db(&state)?;
    let mut handles = Vec::with_capacity(db.chunk_count());
    let (send, recv) = mpsc::channel::<Result<Vec<RecipeId>, String>>();
    // spawn the receiver thread
    state.executor.background_pool().execute(move || {
        let mut result_recipes = Vec::new();
        let mut actors = EnumMap::<Actor, usize>::from_fn(|_| 0);
        for result in recv {
            match result {
                Ok(recipes) => {
                    info!("received {} recipes", recipes.len());
                    for recipe in &recipes {
                        let inputs: RecipeInputs = (*recipe).into();
                        for group in inputs.as_slice() {
                            for actor in group.actors() {
                                actors[*actor] += 1;
                            }
                        }
                    }
                    result_recipes.extend(recipes);
                },
                Err(err) => {
                    error!("error while searching: {}", err);
                    emit_search_complete(&app, ResultInterop::err(err));
                    // app owns aborting. we don't abort easily from a worker thread.
                    return;
                }
            }
        }
        info!("search complete, {} recipes found", result_recipes.len());
        if let Err(e) = save_search_result(&result_recipes) {
            error!("error while saving search result: {}", e);
            emit_search_complete(&app, ResultInterop::err(e.to_string()));
            return;
        }
        let actors = actors.into_values().collect();
        emit_search_complete(&app, ResultInterop::ok(SearchComplete {
            result_count: result_recipes.len(),
            actors,
        }));
    });
    for chunk_id in 0..db.chunk_count() {
        let chunk = db.open_filtered_chunk(chunk_id, filter)?;
        // skip chunks filtered by index
        let chunk = match chunk {
            Some(chunk) => chunk,
            None => {
                info!("skipping chunk {} (filtered by index)", chunk_id);
                continue;
            }
        };
        let send = send.clone();
        let handle = state.executor.execute_abortable(move |abort| {
            let mut result = Vec::new();
            let mut error = None;
            let mut aborted = false;
            let mut last_check_abort_recipe_id = 0;
            for record in chunk {
                let record = match record {
                    Ok(record) => record,
                    Err(err) => {
                        error = Some(err);
                        break;
                    }
                };
                let recipe_id: usize = record.recipe_id.into();
                // was abort requested?
                if recipe_id - last_check_abort_recipe_id >= 10000 {
                    if abort.try_recv().is_ok() {
                        aborted = true;
                        break;
                    }
                    last_check_abort_recipe_id = recipe_id;
                }
                result.push(record.recipe_id);
            }
            if aborted || abort.try_recv().is_ok() {
                return;
            }
            if let Some(err) = error {
                let _ = send.send(Err(err.to_string()));
            }
            let _ = send.send(Ok(result));
        })?;
        handles.push(handle);
    }
    drop(send);

    return Ok(handles);
}

/// Execute filtering on the search result based on what actors should be included.
/// The filter results are returned through the `filter-complete` event.
/// 
/// JS side must ensure that only one filtering is being executed at a time (i.e.
/// `filter_actors` should not be called before `filter-complete` event is received)
#[tauri::command]
fn filter_actors(filter: Vec<usize>, app: AppHandle, state: State<Global>) -> ResultInterop<()> {
    let mut filter = filter.into_iter().map(Actor::from_usize).collect::<HashSet<_>>();
    // always include "none" i.e. the empty space
    filter.insert(Actor::None);
    filter_actors_impl(&Arc::new(filter), app, state).into()
}

fn filter_actors_impl(filter: &Arc<HashSet<Actor>>, app: AppHandle, state: State<Global>) -> Result<(), Error> {
    info!("filtering actors: {:?}", filter);
    let mut filter_state = state.filter.write()?;
    // if the new filter is a subset of the last filter, we can reuse the results
    if filter_state.is_result_loaded &&   filter.intersection(&filter_state.last_filter).copied().collect::<HashSet<_>>() == **filter {
            info!("reusing filtered results from last filtering");
            // use the filtered results
            let results = std::mem::take(&mut filter_state.filtered_results);
            filter_actors_with_iter(results, filter, app, &state)?;

    } else {
        info!("filtering from scratch from search result");
        // load the filtered results from saved search result
        let reader = file_io::open_search_result()?;
        filter_actors_with_iter(
            reader.map_while(|x| x.ok()), 
            filter, app, &state)?;

    }
    Ok(())
}

fn filter_actors_with_iter<I: IntoIterator<Item=RecipeId>>(
    iter: I, filter: &Arc<HashSet<Actor>>, app: AppHandle, state: &State<Global>
) -> Result<(), Error> {
    // chunking the recipes to 4096 to reduce scheduling overhead
    const CHUNK_SIZE: usize = 4096;
    let (send, recv) = mpsc::channel::<Result<Vec<RecipeInfo>, String>>();
    {
        let filter_state = Arc::clone(&state.filter);
        let filter = Arc::clone(filter);
        state.executor.background_pool().execute(move || {
            let mut recipes = Vec::new();
            for result in recv {
                match result {
                    Ok(filtered) => {
                        recipes.extend(filtered);
                    },
                    Err(err) => {
                        emit_filter_complete(&app, ResultInterop::err(err));
                        return;
                    }
                }
            }
            // update filter_state on completion
            {
                if let Ok(mut filter_state) = filter_state.write() {
                    filter_state.is_result_loaded = true;
                    filter_state.filtered_results.clear();
                    filter_state.filtered_results.extend(recipes.iter().map(|info| info.recipe_id));
                    filter_state.last_filter.clear();
                    filter_state.last_filter.extend(filter.iter().copied());
                }
            }
            info!("filtering complete, {} recipes found", recipes.len());
            emit_filter_complete(&app, ResultInterop::ok(FilterComplete {
                results: recipes,
            }));
        });
    }
    for chunk in &iter.into_iter().chunks(CHUNK_SIZE) {
        let chunk: Vec<RecipeId> = chunk.collect();
        let filter = Arc::clone(filter);
        let send = send.clone();
        let pot = get_db(state)?.pot();
        state.executor.pool().execute(move || {
            let mut filtered = Vec::with_capacity(CHUNK_SIZE);
            for recipe_id in chunk {
                // add the recipe if any actor in the recipe is in the filter
                let inputs: RecipeInputs = recipe_id.into();
                for group in inputs.as_slice() {
                    if group.actors().iter().any(|actor| filter.contains(actor)) {
                        // cook me
                        let result = match pot.cook_inputs(recipe_id) {
                            Ok(result) => result,
                            Err(err) => {
                                let _ = send.send(Err(err.to_string()));
                                return;
                            }
                        };

                        let mut group_ids = [0usize; rdata::NUM_INGR];
                        for (i, group) in inputs.as_slice().iter().enumerate() {
                            group_ids[i] = *group as usize;
                        }

                        let info = RecipeInfo {
                            recipe_id,
                            groups: group_ids,
                            value: result.data.health_recover,
                            price: result.data.sell_price,
                        };
                        
                        filtered.push(info);
                        break;
                    }
                }
            }
            let _ = send.send(Ok(filtered));
        });
    }
    drop(send);
    Ok(())
}


fn main() {
    env_logger::init();
    info!("starting application");

    let path = Path::new(".").canonicalize().unwrap();
    info!("current working directory: {}", path.display());

    tauri::Builder::default()
    .manage(Global {
        executor: Executor::new(),
        db: Arc::new(file_io::create_database()),
        filter: Arc::new(RwLock::new(FilterState::default())),

    })
        .invoke_handler(tauri::generate_handler![
            initialize,
            abort,
            search,
            filter_actors
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

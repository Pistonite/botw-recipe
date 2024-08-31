use std::{
    sync::{mpsc, Arc},
    thread,
    time::Duration,
};

use enum_map::EnumMap;
use log::{debug, error, info};
use rdata::db::{Filter, FilteredChunk, TempResult, TempResultReader, TempResultWriter};
use rdata::Actor;
use rdata::RecipeInputs;
use tauri::{AppHandle, State};

use crate::{error::Error, executor::Executor};
use crate::{events, Global};

const ACTOR_META_TIMEOUT: u64 = 1000;

pub fn run(filter: &Filter, app: AppHandle, state: State<Global>) -> Result<Vec<usize>, Error> {
    info!("searching with filter: {:#?}", filter);
    state.executor.clear_abort_handles()?;
    let db = state.get_db()?;
    let temp_result = {
        let mut temp_result = state.search_result.lock()?;
        match temp_result.take() {
            Some(mut temp_result) => {
                info!("reusing existing temp result");
                temp_result.clear()?;
                temp_result
            }
            None => db.new_temporary()?,
        }
    };
    let mut handles = Vec::with_capacity(db.chunk_count());
    // channel for workers to send the result statistics
    let (send, recv) = mpsc::channel::<SearchMsg>();
    // total search space count used to calculate progress
    let mut total_recipe_count = 0;
    let mut file_id = 0;
    for chunk_id in 0..db.chunk_count() {
        let chunk = db.open_filtered_chunk(chunk_id, filter)?;
        // skip chunks filtered by index
        let chunk = match chunk {
            Some(chunk) => chunk,
            None => {
                debug!("skipping chunk {} (filtered by index)", chunk_id);
                continue;
            }
        };
        // open result writer
        let writer = temp_result.writer(file_id)?;
        file_id += 1;
        let chunk_size = rdata::get_compact_chunk_record_size(chunk_id);
        total_recipe_count += chunk_size;
        let send = send.clone();
        let handle = search_task(&state.executor, chunk, writer, send, chunk_size)?;
        handles.push(handle);
    }
    drop(send);
    recv_task(state, &handles, recv, total_recipe_count, app, temp_result);
    return Ok(handles);
}

enum SearchMsg {
    Ok(usize, usize),
    Err(Error),
    Abort,
}

fn search_task(
    executor: &Executor,
    chunk: FilteredChunk,
    mut writer: TempResultWriter,
    send: mpsc::Sender<SearchMsg>,
    chunk_size: usize,
) -> Result<usize, Error> {
    let handle = executor.execute_abortable(move |abort| {
        if abort.try_recv().is_ok() {
            let _ = send.send(SearchMsg::Abort);
            return;
        }
        for record in chunk {
            let record = match record {
                Ok(record) => record,
                Err(err) => {
                    let _ = send.send(SearchMsg::Err(err.into()));
                    return;
                }
            };
            if let Err(e) = writer.write(record.recipe_id) {
                let _ = send.send(SearchMsg::Err(e.into()));
                return;
            }
            // TODO: abort
            // let recipe_id: usize = record.recipe_id.into();
            // was abort requested?
            // if recipe_id % 10000 == 0 {
            // if abort.try_recv().is_ok() {
            //     return;
            // }
            // last_check_abort_recipe_id = recipe_id;
            // }
            // result.push(record.recipe_id);
        }
        let _ = send.send(SearchMsg::Ok(writer.size(), chunk_size));
    })?;

    Ok(handle)
}

fn recv_task(
    state: State<Global>,
    handles: &[usize],
    recv: mpsc::Receiver<SearchMsg>,
    total_recipe_count: usize,
    app: AppHandle,
    mut temp_result: TempResult,
) {
    let executor2 = Arc::clone(&state.executor);
    let handles = handles.to_vec();
    let search_result = Arc::clone(&state.search_result);
    state.executor.background_pool().execute(move || {
        let mut last_percentage = 0;
        let mut searched_count = 0;
        let mut found_count = 0;
        for msg in recv {
            match msg {
                SearchMsg::Ok(found, searched) => {
                    searched_count += searched;
                    found_count += found;
                    let progress =
                        (searched_count as f64 / total_recipe_count as f64 * 100.0) as u32;
                    if progress != last_percentage {
                        info!(
                            "search progress: ({searched_count}/{total_recipe_count}) {progress}%"
                        );
                        last_percentage = progress;
                        events::emit_search_progress(&app, progress);
                    }
                }
                SearchMsg::Err(err) => {
                    error!("error while searching: {}", err);
                    events::emit_search_complete_err(&app, err);
                    for handle in handles {
                        let _ = executor2.abort(handle);
                    }
                    return;
                }
                SearchMsg::Abort => {
                    info!("search aborted");
                    events::emit_search_complete_err(&app, Error::SearchAborted);
                    return;
                }
            }
        }
        info!("search complete, {} recipes found.", found_count);
        info!("processing actor metadata with timeout");
        // read temp result back from disk
        let (send, recv) = mpsc::channel::<ActorMetaMsg>();
        let mut has_error = false;
        let mut actor_meta_handles = Vec::new();
        for result in temp_result.iter() {
            let reader = match result {
                Ok(reader) => reader,
                Err(e) => {
                    error!("error while reading temp result: {}", e);
                    has_error = true;
                    break;
                }
            };
            let send = send.clone();
            let handle = match read_actor_meta(&executor2, send, reader) {
                Ok(handle) => handle,
                Err(e) => {
                    error!("error while spawning actor meta task: {}", e);
                    has_error = true;
                    break;
                }
            };
            actor_meta_handles.push(handle);
        }
        drop(send);
        temp_result.set_size(found_count);
        {
            let mut search_result = match search_result.lock() {
                Ok(x) => x,
                Err(e) => {
                    error!("error while locking temp result: {}", e);
                    events::emit_search_complete_err(&app, e);
                    return;
                }
            };
            *search_result = Some(temp_result);
        }
        if has_error {
            info!("error while reading temp result, aborting actor meta processing");
            for handle in &actor_meta_handles {
                let _ = executor2.abort(*handle);
            }
            events::emit_search_complete_no_actors(&app, found_count);
            return;
        }
        let mut actors = EnumMap::<Actor, usize>::from_fn(|_| 0);
        let executor3 = Arc::clone(&executor2);
        // we are already in the background, must spawn another thread
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(ACTOR_META_TIMEOUT));
            for handle in actor_meta_handles {
                let _ = executor3.abort(handle);
            }
        });
        for msg in recv {
            match msg {
                ActorMetaMsg::Ok(actor) => {
                    for (a, count) in actor {
                        actors[a] += count;
                    }
                }
                ActorMetaMsg::Err(e) => {
                    error!("error while receiving actor meta from worker: {}", e);
                    events::emit_search_complete_no_actors(&app, found_count);
                    return;
                }
                ActorMetaMsg::Abort => {
                    info!("actor meta processing timed out");
                    events::emit_search_complete_no_actors(&app, found_count);
                    return;
                }
            }
        }
        info!("actor metadata processed, emitting search complete");
        events::emit_search_complete(&app, found_count, &actors);
    });
}

enum ActorMetaMsg {
    Ok(EnumMap<Actor, usize>),
    Err(Error),
    Abort,
}

fn read_actor_meta(
    executor: &Executor,
    send: mpsc::Sender<ActorMetaMsg>,
    reader: TempResultReader,
) -> Result<usize, Error> {
    Ok(executor.execute_abortable(move |abort| {
        if abort.try_recv().is_ok() {
            let _ = send.send(ActorMetaMsg::Abort);
            return;
        }
        let mut actors = EnumMap::<Actor, usize>::from_fn(|_| 0);
        for recipe in reader {
            let recipe = match recipe {
                Ok(recipe) => recipe,
                Err(e) => {
                    let _ = send.send(ActorMetaMsg::Err(e.into()));
                    return;
                }
            };
            let inputs: RecipeInputs = recipe.into();
            for group in inputs.as_slice() {
                for actor in group.actors() {
                    actors[*actor] += 1;
                }
            }
        }
        let _ = send.send(ActorMetaMsg::Ok(actors));
    })?)
}

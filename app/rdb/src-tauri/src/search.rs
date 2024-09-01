use std::{
    sync::{mpsc, Arc},
    thread,
    time::Duration,
};

use enum_map::EnumMap;
use log::{debug, error, info};
use rdata::{
    db::{Filter, TempResult},
    Group,
};
use tauri::{AppHandle, State};

use crate::{
    error::Error,
    tasks::{self, CountMsg, ProgressTracker, StatMsg},
};
use crate::{events, Global};

const ACTOR_META_TIMEOUT: u64 = 1000;

pub fn abort(state: State<Global>) -> Result<(), Error> {
    let mut handles = state.search_handles.lock()?;
    for handle in handles.iter() {
        state.executor.abort(*handle)?;
    }
    handles.clear();
    Ok(())
}

pub fn run(filter: &Filter, app: AppHandle, state: State<Global>) -> Result<(), Error> {
    info!("searching with filter: {:#?}", filter);
    let mut handles = state.search_handles.lock()?;
    {
        let mut filter_handles = state.filter_handles.lock()?;
        let mut last_included = state.last_included.lock()?;
        let mut filter_result = state.filter_result.lock()?;
        last_included.clear();
        if let Some(filter_result) = filter_result.take() {
            info!("removing previous filter result");
            let _ = filter_result.remove();
        }
        state.executor.abort_all()?;
        filter_handles.clear();
        handles.clear();
    }
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
    // channel for workers to send the result statistics
    let (send, recv) = mpsc::channel();
    // total search space count used to calculate progress
    let mut total_recipe_count = 0;
    let mut file_id = 0;
    let mut error = None;
    for chunk_id in 0..db.chunk_count() {
        let chunk = match db.open_filtered_chunk(chunk_id, filter) {
            Ok(chunk) => chunk,
            Err(e) => {
                error!("error while opening chunk {}: {}", chunk_id, e);
                error = Some(Error::from(e));
                break;
            }
        };
        // skip chunks filtered by index
        let chunk = match chunk {
            Some(chunk) => chunk,
            None => {
                debug!("skipping chunk {} (filtered by index)", chunk_id);
                continue;
            }
        };
        // open result writer
        let writer = match temp_result.writer(file_id) {
            Ok(writer) => writer,
            Err(e) => {
                error!("error while opening temp result writer: {}", e);
                error = Some(e.into());
                break;
            }
        };
        file_id += 1;
        let chunk_size = chunk.chunk().remaining();
        total_recipe_count += chunk_size;
        let send = send.clone();
        let handle = state.executor.execute_abortable(move |signal| {
            tasks::scan_filtered_chunk(chunk, writer, send, signal)
        });
        let handle = match handle {
            Ok(handle) => handle,
            Err(e) => {
                error!("error while spawning scan task: {}", e);
                error = Some(e);
                break;
            }
        };
        handles.push(handle);
    }
    drop(send);
    if let Some(e) = error {
        info!("error while starting search, aborting");
        for handle in handles.iter() {
            let _ = state.executor.abort(*handle);
        }
        handles.clear();
        return Err(e);
    }
    handle_search_in_background(
        &state,
        handles.clone(),
        recv,
        total_recipe_count,
        app,
        temp_result,
    );

    Ok(())
}

fn handle_search_in_background(
    state: &State<Global>,
    handles: Vec<usize>,
    recv: mpsc::Receiver<CountMsg>,
    total_recipe_count: usize,
    app: AppHandle,
    mut temp_result: TempResult,
) {
    let executor2 = Arc::clone(&state.executor);
    let search_result = Arc::clone(&state.search_result);
    state.executor.continue_in_background(move || {
        let mut progress = ProgressTracker::new(total_recipe_count, |c, t, p| {
            info!("search progress: ({c}/{t}) {p}%");
            events::emit_search_progress(&app, p);
        });
        let mut found_count = 0;
        for msg in recv {
            match msg {
                CountMsg::Ok(found, searched) => {
                    found_count += found;
                    progress.add(searched);
                }
                CountMsg::Err(err) => {
                    error!("error while searching: {}", err);
                    events::emit_search_complete_err(&app, err);
                    for handle in handles {
                        let _ = executor2.abort(handle);
                    }
                    return;
                }
                CountMsg::Abort => {
                    info!("search aborted");
                    events::emit_search_complete_err(&app, Error::Aborted);
                    return;
                }
            }
        }
        info!("search complete, {} recipes found.", found_count);
        temp_result.set_size(found_count);
        info!("stating groups with timeout");
        // read temp result back from disk
        let (send, recv) = mpsc::channel();
        let mut has_error = false;
        let mut stat_handles = Vec::new();
        for reader in temp_result.iter() {
            let reader = match reader {
                Ok(reader) => reader,
                Err(e) => {
                    error!("error while reading temp result: {}", e);
                    has_error = true;
                    break;
                }
            };
            let send = send.clone();
            let result = executor2.execute_abortable(move |signal| {
                tasks::stat_groups(reader, send, signal);
            });
            let handle = match result {
                Ok(handle) => handle,
                Err(e) => {
                    error!("error while spawning stat group task: {}", e);
                    has_error = true;
                    break;
                }
            };
            stat_handles.push(handle);
        }
        drop(send);
        {
            // store the search result
            match search_result.lock() {
                Ok(mut search_result) => {
                    info!("storing search result");
                    *search_result = Some(temp_result);
                }
                Err(e) => {
                    error!("error while locking temp result: {}", e);
                    events::emit_search_complete_err(&app, e);
                    return;
                }
            }
        }
        if has_error {
            info!("error while reading temp result, aborting actor meta processing");
            for handle in &stat_handles {
                let _ = executor2.abort(*handle);
            }
            events::emit_search_complete_no_stat(&app, found_count);
            return;
        }
        let executor3 = Arc::clone(&executor2);
        // we are already in the background, must spawn another thread
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(ACTOR_META_TIMEOUT));
            for handle in stat_handles {
                let _ = executor3.abort(handle);
            }
        });
        let mut groups = EnumMap::<Group, usize>::from_fn(|_| 0);
        for msg in recv {
            match msg {
                StatMsg::Ok(group) => {
                    for (a, count) in group {
                        groups[a] += count;
                    }
                }
                StatMsg::Err(e) => {
                    error!("error while receiving group stat from worker: {}", e);
                    // let the timeout handle the abort
                    events::emit_search_complete_no_stat(&app, found_count);
                    return;
                }
                StatMsg::Abort => {
                    info!("stat group timed out, emitting result with no stat");
                    events::emit_search_complete_no_stat(&app, found_count);
                    return;
                }
            }
        }
        info!("stat group successful, emitting search complete");
        events::emit_search_complete(&app, found_count, &groups);
    });
}

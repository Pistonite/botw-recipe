use std::sync::Arc;
use std::{collections::HashSet, sync::mpsc};

use enum_map::EnumMap;
use log::{error, info};
use rdata::db::TempResult;
use rdata::Group;
use tauri::{AppHandle, State};

use crate::events;
use crate::tasks::{self, ProgressTracker, StatCountMsg};
use crate::{error::Error, Global};

pub fn abort(state: State<Global>) -> Result<(), Error> {
    let mut handles = state.filter_handles.lock()?;
    for handle in handles.iter() {
        state.executor.abort(*handle)?;
    }
    handles.clear();
    Ok(())
}

pub fn run(
    next_included: &Arc<HashSet<Group>>,
    app: AppHandle,
    state: State<Global>,
) -> Result<(), Error> {
    info!("filtering groups: {:?}", next_included);
    let mut handles = state.filter_handles.lock()?;
    state.executor.abort_all()?;
    handles.clear();
    let db = state.get_db()?;
    let last_included = {
        let mut set = state.last_included.lock()?;
        std::mem::take(&mut *set)
    };
    // if the new filter is a subset of the last filter, we can reuse the results
    let temp_result = if next_included
        .iter()
        .all(|group| last_included.contains(group))
    {
        get_last_filter_result(&state)?
    } else {
        info!("filtering from scratch from search result");
        get_search_result(&state)?
    };
    let new_temp_result = db.new_temporary()?;
    let (send, recv) = mpsc::channel();
    let mut error = None;
    for (file_id, reader) in temp_result.iter().enumerate() {
        let reader = match reader {
            Ok(reader) => reader,
            Err(e) => {
                error!("error while reading temp result: {}", e);
                error = Some(Error::from(e));
                break;
            }
        };
        let send = send.clone();
        let writer = match new_temp_result.writer(file_id) {
            Ok(writer) => writer,
            Err(e) => {
                error!("error while creating temp result writer: {}", e);
                error = Some(e.into());
                break;
            }
        };
        let next_included = Arc::clone(next_included);
        let handle = state.executor.execute_abortable(move |signal| {
            tasks::filter_and_stat_groups(reader, writer, next_included, send, signal);
        });
        let handle = match handle {
            Ok(handle) => handle,
            Err(e) => {
                error!("error while executing filter task: {}", e);
                error = Some(e);
                break;
            }
        };
        handles.push(handle);
    }
    drop(send);
    if let Some(e) = error {
        info!("error while starting filter, aborting");
        for handle in handles.iter() {
            let _ = state.executor.abort(*handle);
        }
        handles.clear();
        return Err(e);
    }
    handle_filter_in_background(
        &state,
        handles.clone(),
        recv,
        temp_result.get_size(),
        app,
        new_temp_result,
    );
    Ok(())
}

fn get_last_filter_result(state: &State<Global>) -> Result<TempResult, Error> {
    {
        let filter_result = state.filter_result.lock()?;
        if let Some(temp_result) = filter_result.as_ref() {
            info!("reusing filtered results from last filtering");
            return Ok(temp_result.clone());
        }
    }
    error!("no filtered results found, filtering from scratch (this should NOT happen!!)");
    get_search_result(state)
}

fn get_search_result(state: &State<Global>) -> Result<TempResult, Error> {
    let temp_result = state.search_result.lock()?;
    match temp_result.as_ref() {
        Some(temp_result) => Ok(temp_result.clone()),
        None => {
            error!("search result not found. This should not happen");
            Err(Error::MissingSearchResult)
        }
    }
}

fn handle_filter_in_background(
    state: &State<Global>,
    handles: Vec<usize>,
    recv: mpsc::Receiver<StatCountMsg>,
    total: usize,
    app: AppHandle,
    mut temp_result: TempResult,
) {
    let executor2 = Arc::clone(&state.executor);
    let filter_result = Arc::clone(&state.filter_result);
    let last_included = Arc::clone(&state.last_included);
    state.executor.continue_in_background(move || {
        let mut progress = ProgressTracker::new(total, |c, t, p| {
            info!("filter progress: ({c}/{t}) {p}%");
            events::emit_filter_progress(&app, p);
        });
        let mut found_count = 0;
        let mut groups = EnumMap::<Group, usize>::from_fn(|_| 0);
        let mut included = HashSet::new();
        for msg in recv {
            match msg {
                StatCountMsg::Ok(found, total, group_stat) => {
                    found_count += found;
                    progress.add(total);
                    for (group, count) in group_stat.into_iter() {
                        groups[group] += count;
                        included.insert(group);
                    }
                }
                StatCountMsg::Err(err) => {
                    error!("error while filtering: {}", err);
                    for handle in handles {
                        let _ = executor2.abort(handle);
                    }
                    events::emit_filter_complete_err(&app, err);
                    return;
                }
                StatCountMsg::Abort => {
                    info!("filter aborted");
                    events::emit_filter_complete_err(&app, Error::Aborted);
                    return;
                }
            }
        }
        info!("filtering complete, found {} recipes", found_count);
        temp_result.set_size(found_count);
        {
            match last_included.lock() {
                Ok(mut last_included) => {
                    let _ = std::mem::replace(&mut *last_included, included);
                }
                Err(e) => {
                    error!("error while updating last included groups: {}", e);
                    events::emit_filter_complete_err(&app, e);
                    return;
                }
            }
        }
        {
            match filter_result.lock() {
                Ok(mut filter_result) => {
                    if let Some(filter_result) = filter_result.take() {
                        if let Err(e) = filter_result.remove() {
                            error!("error while removing old filter result: {}", e);
                            // let it continue since temporaries are cleaned up later anyway
                        }
                    }
                    *filter_result = Some(temp_result);
                }
                Err(e) => {
                    error!("error while updating filter result: {}", e);
                    events::emit_filter_complete_err(&app, e);
                    return;
                }
            }
        }
        info!("emitting filter complete");
        events::emit_filter_complete(&app, found_count, &groups);
    });
}

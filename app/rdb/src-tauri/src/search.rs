use std::sync::{mpsc, Arc};

use enum_map::EnumMap;
use log::{debug, info, error};
use rdata::{db::{Database, Filter, FilteredChunk, TempResult, TempResultReader, TempResultWriter}, recipe::RecipeInputs, Actor, Executor};
use tauri::{AppHandle, State};

use crate::{error::{Error, ResultInterop}, events, Global};


pub fn run(filter: &Filter, app: AppHandle, state: State<Global>) -> Result<Vec<usize>, Error> {
    info!("searching with filter: {:#?}", filter);
    state.executor.clear_abort_handles()?;
    let db = state.get_db()?;
    let temp_result = db.new_temporary()?;
    let mut handles = Vec::with_capacity(db.chunk_count());
    // channel for workers to send the result statistics
    let (send, recv) = mpsc::channel::<Result<(usize, usize), Error>>();
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
    recv_task(&state.executor, &handles, recv, total_recipe_count, app, temp_result);
    return Ok(handles);
}

fn search_task(
    executor: &Executor,
    chunk: FilteredChunk,
    mut writer: TempResultWriter, 
    send: mpsc::Sender<Result<(usize, usize), Error>>, 
    chunk_size: usize
) -> Result<usize, Error> {
    let handle = executor.execute_abortable(move |abort| {
        for record in chunk {
            let record = match record {
                Ok(record) => record,
                Err(err) => {
                    let _ = send.send(Err(err.into()));
                    return;
                }
            };
            if let Err(e) = writer.write(record.recipe_id) {
                let _ = send.send(Err(e.into()));
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
        let _ = send.send(Ok((writer.size(), chunk_size)));
    })?;

    Ok(handle)
}

fn recv_task(
    executor: &Arc<Executor>,
    handles: &[usize],
    recv: mpsc::Receiver<Result<(usize, usize), Error>>,
    total_recipe_count: usize,
    app: AppHandle,
    temp_result: TempResult,
) {
    let executor2 = Arc::clone(&executor);
    let handles = handles.to_vec();
    executor.background_pool().execute(move || {
        let mut last_percentage = 0;
        let mut searched_count = 0;
        let mut found_count = 0;
        for result in recv {
            match result {
                Ok((found, searched)) => {
                    searched_count += searched;
                    found_count += found;
                    let progress = (searched_count as f64 / total_recipe_count as f64 * 100.0) as u32;
                    if progress != last_percentage {
                        info!("search progress: ({searched_count}/{total_recipe_count}) {progress}%");
                        last_percentage = progress;
                        events::emit_search_progress(&app, progress);
                    }
                }
                Err(err) => {
                    error!("error while searching: {}", err);
                    events::emit_search_complete(&app, ResultInterop::err(err));
                    for handle in handles {
                        let _ = executor2.abort(handle);
                    }
                    return;
                }
            }
        }
        info!("search complete, {} recipes found. processing actor metadata", found_count);
        // read temp result back from disk
        let (send, recv) = mpsc::channel::<Result<EnumMap<Actor, usize>, Error>>();
        for result in temp_result.iter() {
            let reader = match result {
                Ok(reader) => reader,
                Err(e) => {
                    error!("error while reading temp result: {}", e);
                    continue;
                }
            };
            let send = send.clone();
            read_actor_meta(&executor2, send, reader);
        }
        drop(send);
        let mut actors = EnumMap::<Actor, usize>::from_fn(|_| 0);
        for result in recv {
            match result {
                Ok(actor) => {
                    for (a, count) in actor {
                        actors[a] += count;
                    }
                }
                Err(e) => {
                    error!("error while reading actor meta: {}", e);
                    events::emit_search_complete(&app, ResultInterop::err(e));
                    return;
                }
            }
        }
        info!("actor metadata processed, emitting search complete");
        let actors = actors.into_values().collect();
        events::emit_search_complete(
            &app,
            ResultInterop::ok(events::SearchComplete {
                found_count,
                actors,
            }),
        );
    });
}

fn read_actor_meta(
    executor: &Executor,
    send: mpsc::Sender<Result<EnumMap<Actor, usize>, Error>>,
    reader: TempResultReader,
) {
    executor.pool().execute(move || {
        let mut actors = EnumMap::<Actor, usize>::from_fn(|_| 0);
        for recipe in reader {
            let recipe = match recipe {
                Ok(recipe) => recipe,
                Err(e) => {
                    let _ = send.send(Err(e.into()));
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
        let _ = send.send(Ok(actors));
    })
}

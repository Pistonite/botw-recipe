use std::collections::HashSet;
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};

use enum_map::EnumMap;
use rdata::db::{FilteredChunk, TempResultReader, TempResultWriter};
use rdata::{Group, RecipeInputs};

use crate::error::Error;
use crate::executor::AbortSignal;

/// Message for counting
pub enum CountMsg {
    /// Finished a job, returns (`found`, `total`)
    Ok(usize, usize),
    /// Encountered an error
    Err(Error),
    /// Abort the task
    Abort,
}

/// Message for stating groups in a recipe set
pub enum StatMsg {
    /// Finished stating, returns occurence of each group
    Ok(EnumMap<Group, usize>),
    /// Encountered an error
    Err(Error),
    /// Abort the task
    Abort,
}

/// Message for counting and stating groups in a recipe set
pub enum StatCountMsg {
    /// Finished stating, returns `found`, `total`, occurence of each group
    Ok(usize, usize, EnumMap<Group, usize>),
    /// Encountered an error
    Err(Error),
    /// Abort the task
    Abort,
}

/// Task to scan recipes in a `FilteredChunk`.
///
/// Writes result to `writer`, and send a `CountMst` to `send` when finished.
/// Checks `signal` before starting the scan. Aborting after scanning
/// started will not stop the scan.
pub fn scan_filtered_chunk(
    chunk: FilteredChunk,
    mut writer: TempResultWriter,
    send: mpsc::Sender<CountMsg>,
    signal: AbortSignal,
) {
    if signal.is_aborted() {
        let _ = send.send(CountMsg::Abort);
        return;
    }
    let chunk_size = chunk.chunk().remaining();
    for record in chunk {
        let record = match record {
            Ok(record) => record,
            Err(err) => {
                let _ = send.send(CountMsg::Err(err.into()));
                return;
            }
        };
        if let Err(e) = writer.write(record.recipe_id) {
            let _ = send.send(CountMsg::Err(e.into()));
            return;
        }
    }
    let _ = send.send(CountMsg::Ok(writer.size(), chunk_size));
}

/// Task to scan a temp result and count the occurence of each group.
///
/// Send a `StatMsg` to `send` when finished.
/// Checks `signal` before starting the scan. Aborting after scanning
/// started will not stop the scan.
pub fn stat_groups(reader: TempResultReader, send: mpsc::Sender<StatMsg>, signal: AbortSignal) {
    if signal.is_aborted() {
        let _ = send.send(StatMsg::Abort);
        return;
    }
    let mut groups = EnumMap::<Group, usize>::from_fn(|_| 0);
    for recipe in reader {
        let recipe = match recipe {
            Ok(recipe) => recipe,
            Err(e) => {
                let _ = send.send(StatMsg::Err(e.into()));
                return;
            }
        };
        let inputs: RecipeInputs = recipe.into();
        for group in inputs.as_slice() {
            groups[*group] += 1;
        }
    }
    let _ = send.send(StatMsg::Ok(groups));
}

/// Task to scan a temp result, count the occurence of each group, and filter
/// recipes with all ingredients in `included`.
pub fn filter_and_stat_groups(
    reader: TempResultReader,
    mut writer: TempResultWriter,
    included: Arc<HashSet<Group>>,
    send: mpsc::Sender<StatCountMsg>,
    signal: AbortSignal,
) {
    if signal.is_aborted() {
        let _ = send.send(StatCountMsg::Abort);
        return;
    }
    let mut groups = EnumMap::<Group, usize>::from_fn(|_| 0);
    let mut total = 0;
    for recipe in reader {
        total += 1;
        let recipe = match recipe {
            Ok(recipe) => recipe,
            Err(e) => {
                let _ = send.send(StatCountMsg::Err(e.into()));
                return;
            }
        };
        let inputs: RecipeInputs = recipe.into();
        let should_include = inputs
            .as_slice()
            .iter()
            .all(|group| included.contains(group));
        if !should_include {
            continue;
        }
        if let Err(e) = writer.write(recipe) {
            let _ = send.send(StatCountMsg::Err(e.into()));
            return;
        }
        for group in inputs.as_slice() {
            groups[*group] += 1;
        }
    }
    let _ = send.send(StatCountMsg::Ok(writer.size(), total, groups));
}

pub struct ProgressTracker<F> {
    total: usize,
    current: usize,
    last_update_time: Instant,
    last_percentage: u32,
    update_fn: F,
}

impl<F> ProgressTracker<F>
where
    F: Fn(usize, usize, u32),
{
    pub fn new(total: usize, update_fn: F) -> Self {
        Self {
            total,
            current: 0,
            last_update_time: Instant::now(),
            last_percentage: 0,
            update_fn,
        }
    }

    pub fn add(&mut self, count: usize) {
        self.current += count;
        let progress = (self.current as f64 / self.total as f64 * 100.0) as u32;
        let should_update = if self.current >= self.total {
            true
        } else if progress != self.last_percentage {
            self.last_update_time.elapsed() > Duration::from_secs(1)
        } else {
            false
        };
        if !should_update {
            return;
        }
        self.last_update_time = Instant::now();
        self.last_percentage = progress;
        (self.update_fn)(self.current, self.total, progress);
    }
}

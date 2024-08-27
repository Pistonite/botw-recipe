//! Executor implementation without a full-blown async runtime

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

use derivative::Derivative;
use oneshot::{Receiver, Sender};
use threadpool::ThreadPool;

use crate::error::Error;

pub struct Executor {
    pool: ThreadPool,
    bg_pool: ThreadPool,
    next_abortable_id: AtomicUsize,
    abort_senders: Arc<RwLock<HashMap<usize, Abort>>>,
    abort_finished: Arc<RwLock<HashSet<usize>>>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            pool: ThreadPool::new(num_cpus::get()),
            bg_pool: ThreadPool::new(num_cpus::get() / 2),
            next_abortable_id: AtomicUsize::new(0),
            abort_senders: Arc::new(RwLock::new(HashMap::new())),
            abort_finished: Arc::new(RwLock::new(HashSet::new())),
        }
    }
    /// Get the underlying thread pool for executing non-abortable tasks directly
    #[inline]
    pub fn pool(&self) -> &ThreadPool {
        &self.pool
    }

    #[inline]
    pub fn background_pool(&self) -> &ThreadPool {
        &self.bg_pool
    }

    /// Clear previously finished abortable tasks.
    pub fn clear_abort_handles(&self) -> Result<(), Error> {
        // lock in this order
        let mut senders = self.abort_senders.write()?;
        let mut finished = self.abort_finished.write()?;
        for id in finished.iter() {
            senders.remove(id);
        }
        finished.clear();

        Ok(())
    }

    /// Execute an abortable task.
    ///
    /// The task must have its own implementation of checking the abort signal
    /// through the provided `oneshot::Receiver<()>`.
    pub fn execute_abortable<F>(&self, f: F) -> Result<usize, Error>
    where
        F: FnOnce(Receiver<()>) + Send + 'static,
    {
        let id = self.next_id()?;
        let (send, recv) = oneshot::channel();
        {
            let mut senders = self.abort_senders.write()?;
            senders.insert(id, Abort(send));
        }
        let finished = Arc::clone(&self.abort_finished);

        self.pool.execute(move || {
            // no check here - the task must check the abort signal
            // execute
            f(recv);
            // mark as finish
            finished.write().unwrap().insert(id);
        });

        Ok(id)
    }

    fn next_id(&self) -> Result<usize, Error> {
        let mut id = self.next_abortable_id.fetch_add(1, Ordering::SeqCst);
        let first_id = id;
        {
            let senders = self.abort_senders.read()?;
            while senders.contains_key(&id) {
                id = id.wrapping_add(1);
                if id == first_id {
                    return Err(Error::ExecutorIdUnavailable);
                }
            }
            if id != first_id {
                self.next_abortable_id.store(id, Ordering::SeqCst);
            }
        }
        Ok(id)
    }

    /// Abort a task.
    ///
    /// Does nothing if the task is already completed, or doesn't exist
    pub fn abort(&self, handle_id: usize) -> Result<(), Error> {
        let mut senders = self.abort_senders.write()?;
        if let Some(sender) = senders.remove(&handle_id) {
            let _ = sender.send();
        }
        Ok(())
    }
}

struct Abort(Sender<()>);
impl Abort {
    pub fn send(self) {
        let _ = self.0.send(());
    }
}
// safety: https://github.com/faern/oneshot/issues/26
// as long as we only let one thread access the sender at a time, it's safe
// which is guarded by the RwLock on the map
unsafe impl Sync for Abort {}

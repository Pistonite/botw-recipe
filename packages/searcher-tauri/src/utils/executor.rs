//! Executor implementation without a full-blown async runtime

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

use oneshot::{Receiver, Sender};
use threadpool::ThreadPool;

use crate::Error;

pub struct Executor {
    pool: ThreadPool,
    bg_pool: ThreadPool,
    next_abortable_id: AtomicUsize,
    abort_senders: Arc<RwLock<HashMap<usize, Abort>>>,
    abort_finished: Arc<RwLock<HashSet<usize>>>,
}

impl Executor {
    /// Create a new executor.
    ///
    /// The size hint is usually number of CPUs.
    pub fn new(worker_size_hint: usize) -> Self {
        // minimum 2 workers, minus 2 to allocate some for background tasks
        let main_workers = if worker_size_hint > 2 {
            (worker_size_hint - 2).max(2)
        } else {
            2
        };
        Self {
            pool: ThreadPool::new(main_workers),
            bg_pool: ThreadPool::new(1),
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

    pub fn continue_in_background<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.bg_pool.execute(f);
    }

    pub fn join(&self) {
        self.pool.join();
        self.bg_pool.join();
    }

    /// Clear previously finished abortable tasks.
    pub fn abort_all(&self) -> Result<(), Error> {
        // lock in this order
        let mut senders = self.abort_senders.write()?;
        let mut finished = self.abort_finished.write()?;
        for id in finished.iter() {
            senders.remove(id);
        }
        let senders = std::mem::take(&mut *senders);
        for sender in senders.into_values() {
            sender.send();
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
        F: FnOnce(AbortSignal) + Send + 'static,
    {
        let (send, recv) = oneshot::channel();
        let id = self.add_abort_sender(send)?;
        let finished = Arc::clone(&self.abort_finished);

        self.pool.execute(move || {
            // no check here - the task must check the abort signal
            // execute
            f(AbortSignal(recv));
            // mark as finish
            finished.write().unwrap().insert(id);
        });

        Ok(id)
    }

    fn add_abort_sender(&self, send: Sender<()>) -> Result<usize, Error> {
        let mut id = self.next_abortable_id.fetch_add(1, Ordering::SeqCst);
        let first_id = id;
        {
            let mut senders = self.abort_senders.write()?;
            while senders.contains_key(&id) {
                id = id.wrapping_add(1);
                if id == first_id {
                    return Err(Error::ExecutorUnavailable);
                }
            }
            if id != first_id {
                self.next_abortable_id.store(id, Ordering::SeqCst);
            }
            senders.insert(id, Abort(send));
        }
        Ok(id)
    }

    /// Abort a task.
    ///
    /// Does nothing if the task is already completed, or doesn't exist
    pub fn abort(&self, handle_id: usize) -> Result<(), Error> {
        let mut senders = self.abort_senders.write()?;
        if let Some(sender) = senders.remove(&handle_id) {
            sender.send();
        }
        Ok(())
    }
}

#[repr(transparent)]
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

#[repr(transparent)]
pub struct AbortSignal(Receiver<()>);
impl AbortSignal {
    pub fn is_aborted(&self) -> bool {
        self.0.try_recv().is_ok()
    }
}

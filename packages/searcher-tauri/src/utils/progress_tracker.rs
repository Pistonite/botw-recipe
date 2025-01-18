use std::time::{Duration, Instant};

/// Wrapper for periodically emit update event during a long running process
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
    /// Create a new `ProgressTracker` with total count and update function
    pub fn new(total: usize, update_fn: F) -> Self {
        Self {
            total,
            current: 0,
            last_update_time: Instant::now(),
            last_percentage: 0,
            update_fn,
        }
    }

    /// Add `count` toward the progress
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

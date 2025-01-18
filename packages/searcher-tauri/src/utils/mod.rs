/// Configuration file
mod config;
pub use config::Config;

mod progress_tracker;
pub use progress_tracker::ProgressTracker;
/// Thread pool executor implementation
/// that allows for aborting tasks
mod executor;
pub use executor::{Executor, AbortSignal};


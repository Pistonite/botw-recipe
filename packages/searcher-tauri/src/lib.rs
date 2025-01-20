/// Global state of the searcher
mod global;
pub use global::Global;

/// Error type
mod error;
pub use error::*;

/// Main stages of the app
///
/// These are:
/// - Search: Search for recipes based on modifier
/// - Filter: Filter results based on materials
/// - Cook: Compute and display filtered results
pub mod stages;

/// Tasks invoked from the stages
pub mod tasks;

/// Events emitted to the UI
pub mod events;

/// Commands invocable from UI
pub mod commands;

/// Misc utils
mod utils;
pub use utils::*;

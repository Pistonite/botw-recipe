mod actor;
pub use actor::*;
pub mod constants;
mod cook_item;
pub use cook_item::*;
mod group;
pub use group::*;

/// Get the number of record in a raw chunk
pub fn get_raw_chunk_record_size(chunk_id: usize) -> usize {
    if chunk_id == crate::CHUNK_COUNT - 1 {
        crate::LAST_CHUNK_SIZE
    } else {
        crate::CHUNK_SIZE
    }
}

pub fn get_compact_chunk_record_size(chunk_id: usize) -> usize {
    if chunk_id == crate::COMPACT_CHUNK_COUNT - 1 {
        crate::COMPACT_LAST_CHUNK_SIZE
    } else {
        crate::COMPACT_CHUNK_SIZE
    }
}

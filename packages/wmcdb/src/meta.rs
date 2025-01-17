use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use botw_recipe_cook::CookData;

use super::{Error, Index, Record};

/// Save the database index file to the given path
pub fn save_index(path: impl AsRef<Path>, index: &[Index]) -> Result<(), Error> {
    let writer = BufWriter::new(File::create(path)?);
    serde_yaml_ng::to_writer(writer, index)?;
    Ok(())
}

/// Load the database index file from the given path
pub fn load_index(path: impl AsRef<Path>) -> Result<Vec<Index>, Error> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(Error::MissingIndex);
    }
    let reader = BufReader::new(File::open(path)?);
    let expected_size = compact_v2().chunk_count();
    let index: Vec<Index> = serde_yaml_ng::from_reader(reader)?;
    if index.len() != expected_size as usize {
        return Err(Error::InvalidIndexChunkCount(expected_size, index.len() as u32));
    }
    Ok(index)
}

/// Get the index file path from database directory
pub fn index_path(db_path: impl AsRef<Path>) -> PathBuf {
    db_path.as_ref().join("index.yaml")
}


/// Metadata about the database files
#[derive(Debug, Clone, PartialEq)]
pub struct DbMeta {
    /// If the database is in raw or compact format
    ///
    /// Raw format contains more information about the cook result,
    /// and should be used for validation and comparison. Compact format
    /// is packed to only have the necessary information for WMC search.
    is_raw: bool,

    /// Number of records in each chunk, except for the last chunk
    chunk_size: u32,

    /// Number of chunks in the database
    chunk_count: u32,

    /// Number of total records in the database
    total_record: u64,
}

/// Get the raw chunk path for a given chunk ID in the database directory
#[inline]
pub fn raw_chunk_path(db_path: &Path, chunk_id: impl std::fmt::Display) -> PathBuf {
    db_path.join(format!("chunk_{}.rawdat", chunk_id))
}

/// Get the compact chunk path for a given chunk ID in the database directory
#[inline]
pub fn compact_chunk_path(db_path: &Path, chunk_id: impl std::fmt::Display) -> PathBuf {
    db_path.join(format!("chunk_{}.rdb", chunk_id))
}

/// Get the metadata for RawDB v2
#[inline]
const fn raw_v2() -> DbMeta {
    DbMeta {
        is_raw: true,
        chunk_size: 409600,
        chunk_count: 3534,
        total_record: 1447490660,
    }
}

/// Get the metadata for CompactDB v2
#[inline]
const fn compact_v2() -> DbMeta {
    DbMeta {
        is_raw: false,
        chunk_size: 2048000,
        chunk_count: 707,
        total_record: 1447490660,
    }
}

#[inline]
pub const fn raw() -> DbMeta {
    raw_v2()
}

#[inline]
pub const fn compact() -> DbMeta {
    compact_v2()
}

impl DbMeta {
    /// Get if self is in raw format
    #[inline]
    pub const fn is_raw(&self) -> bool {
        self.is_raw
    }

    /// Get the byte size of a record (one recipe)
    #[inline]
    pub const fn record_size(&self) -> usize {
        if self.is_raw {
            std::mem::size_of::<CookData>()
        } else {
            std::mem::size_of::<Record>()
        }
    }

    /// Get the number of records in a chunk
    #[inline]
    pub const fn chunk_size(&self, chunk_id: u32) -> usize {
        if chunk_id == self.chunk_count - 1 {
            (self.total_record - 
            (self.chunk_size * (self.chunk_count - 1)) as u64) as usize
        } else {
            self.chunk_size as usize
        }
    }

    /// Get the byte size of a chunk
    #[inline]
    pub const fn chunk_size_bytes(&self, chunk_id: u32) -> usize {
        self.chunk_size(chunk_id) * self.record_size()
    }

    /// Get the number of chunks in the database
    #[inline]
    pub const fn chunk_count(&self) -> u32 {
        self.chunk_count
    }

    /// Get the record id range in a chunk [start, end)
    #[inline]
    pub fn record_range(&self, chunk_id: u32) -> (u64, u64) {
        let chunk_size = self.chunk_size as u64;
        let start = (chunk_id as u64) * chunk_size;
        let end = self.total_record.min(start + chunk_size);
        (start, end)
    }

    /// Get the total number of records in the database
    #[inline]
    pub const fn total_record(&self) -> u64 {
        self.total_record
    }
}

#[cfg(test)]
mod tests {
    use super::*;
/// Get the metadata for RawDB V1 (no monster extract)
#[inline]
const fn raw_v1() -> DbMeta {
    DbMeta {
        is_raw: true,
        chunk_size: 409600,
        chunk_count: 4409,
        total_record: 1805568402,
    }
}

/// Get the metadata for CompactDB V1 (no monster extract)
#[inline]
const fn compact_v1() -> DbMeta {
    DbMeta {
        is_raw: false,
        chunk_size: 2048000,
        chunk_count: 882,
        total_record: 1805568402,
    }
}

    #[test]
    fn test_record_sizes() {
        // ensure that record sizes aren't changed unexpectedly
        let meta = raw_v1();
        assert_eq!(meta.record_size(), 24);
        let meta = compact_v1();
        assert_eq!(meta.record_size(), 2);
    }

    #[test]
    fn test_last_chunk_size_raw_v1() {
        // ensure that sizes aren't changed unexpectedly
        let meta = raw_v1();
        assert_eq!(meta.chunk_size(meta.chunk_count - 1), 51602);
    }

    #[test]
    fn test_last_chunk_size_compact_v1() {
        // ensure that sizes aren't changed unexpectedly
        let meta = compact_v1();
        assert_eq!(meta.chunk_size(meta.chunk_count - 1), 1280402);
    }
}

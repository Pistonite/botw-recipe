use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::sync::Arc;

use crate::cook::CookingPot;

use super::{Error, Filter, PositionedRecord, Record};

/// Chunk for sequential access
pub struct Chunk {
    reader: BufReader<File>,
    /// The next ID to read
    recipe_next: u64,
    /// The last ID to read
    recipe_end: u64,
}

impl Chunk {
    /// Open a chunk for reading recipes.
    pub fn open<P: AsRef<Path>>(chunk_id: u32, path: P) -> Result<Self, Error> {
        let file = File::open(path.as_ref())?;
        let meta = crate::fsdb::meta::compact_v2();
        let total = meta.chunk_size(chunk_id);
        let file_size = file.metadata()?.len() as usize;
        if file_size != meta.chunk_size_bytes(chunk_id) {
            return Err(Error::InvalidChunkSize(total * 2, file_size));
        }
        let mut reader = BufReader::new(file);
        let (start, end) = meta.record_range(chunk_id);
        let recipe_next = if chunk_id == 0 {
            // 0 corresponds to 5 of <none>, skip 2 bytes
            reader.read_exact(&mut [0; 2])?;
            1
        } else {
            start
        };
        Ok(Self {
            reader,
            recipe_next,
            recipe_end: end,
        })
    }

    /// Attach a filter to this chunk
    pub fn filter(self, filter: &Filter, pot: Arc<CookingPot>) -> FilteredChunk {
        FilteredChunk::new(self, filter.clone(), pot)
    }

    /// Get the number of remaining records to read
    pub fn remaining(&self) -> usize {
        (self.recipe_end - self.recipe_next) as usize
    }
}

impl Iterator for Chunk {
    type Item = Result<PositionedRecord, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.recipe_next >= self.recipe_end {
            return None;
        }
        let mut buf = [0; 2];
        match self.reader.read_exact(&mut buf) {
            Err(e) => {
                // increment recipe_next so we don't get stuck on
                // the underlying IO error
                self.recipe_next += 1;
                Some(Err(e.into()))
            }
            Ok(_) => {
                let recipe_id = self.recipe_next;
                self.recipe_next += 1;
                Some(Ok(PositionedRecord {
                    recipe_id,
                    record: Record::from_slice(&buf),
                }))
            }
        }
    }
}

pub struct FilteredChunk {
    chunk: Chunk,
    filter: Filter,
    pot: Arc<CookingPot>,
}

impl FilteredChunk {
    pub fn new(chunk: Chunk, filter: Filter, pot: Arc<CookingPot>) -> Self {
        Self { chunk, filter, pot }
    }

    pub fn chunk(&self) -> &Chunk {
        &self.chunk
    }
}

impl Iterator for FilteredChunk {
    type Item = Result<PositionedRecord, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chunk.next() {
                None => return None,
                Some(Err(e)) => return Some(Err(e)),
                Some(Ok(record)) => match record.matches(&self.filter, &self.pot) {
                    Err(e) => return Some(Err(e)),
                    Ok(false) => continue,
                    Ok(true) => {
                        return Some(Ok(record));
                    }
                },
            }
        }
    }
}

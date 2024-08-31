use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::sync::Arc;

use crate::cook::CookingPot;
use crate::generated::get_compact_chunk_record_size;
use crate::recipe::RecipeId;

use super::{Error, Filter, PositionedRecord};

/// Chunk for sequential access
pub struct Chunk {
    reader: BufReader<File>,
    recipe_next: usize,
    recipe_end: usize,
}

impl Chunk {
    pub fn open<P: AsRef<Path>>(chunk_id: usize, path: P) -> Result<Self, Error> {
        let file = File::open(path.as_ref())?;
        let total = get_compact_chunk_record_size(chunk_id);
        let file_size = file.metadata()?.len() as usize;
        if file_size != total * 2 {
            return Err(Error::InvalidChunkSize(total * 2, file_size));
        }
        let mut reader = BufReader::new(file);
        let recipe_next = if chunk_id == 0 {
            // 0 corresponds to 5 of <none>, skip 2 bytes
            reader.read_exact(&mut [0; 2])?;
            1
        } else {
            chunk_id * crate::COMPACT_CHUNK_SIZE
        };
        Ok(Self {
            reader,
            recipe_next,
            recipe_end: chunk_id * crate::COMPACT_CHUNK_SIZE + total,
        })
    }

    pub fn filter(self, filter: &Filter, pot: Arc<CookingPot>) -> FilteredChunk {
        FilteredChunk::new(self, filter.clone(), pot)
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
                    recipe_id: RecipeId::new_unchecked(recipe_id),
                    record: u16::from_le_bytes(buf).into(),
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

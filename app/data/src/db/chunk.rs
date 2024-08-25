use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    sync::Arc,
};

use crate::{cook::CookingPot, generated::get_compact_chunk_record_size, recipe::RecipeId};

use super::{Error, Filter, PositionedRecord, Record};

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
            return Err(Error::InvalidDatabase(format!(
                "wrong chunk size: expected {}, actual {}",
                total * 2,
                file_size
            )));
        }
        Ok(Self {
            reader: BufReader::new(file),
            recipe_next: chunk_id * crate::COMPACT_CHUNK_SIZE,
            recipe_end: total,
        })
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
            Err(e) => Some(Err(e.into())),
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

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::cook::CookingPot;

use super::{Chunk, Error, Filter, FilteredChunk, Index};

/// Main database handle
pub struct Database {
    /// Path to the database folder (containg index.yaml, the chunks, etc)
    path: PathBuf,
    /// The index data. i-th element corresponds to the i-th chunk
    index: Box<[Index]>,
    /// The cooker, in case we need more information on crit
    pot: Arc<CookingPot>,
}

impl Database {
    /// Open a database and loads the index data and crit.db
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref().to_path_buf();
        let index_path = path.join("index.yaml");
        if !index_path.exists() {
            return Err(Error::InvalidDatabase("index.yaml not found".into()));
        }
        let reader = BufReader::new(File::open(&index_path)?);
        let index: Vec<Index> = serde_yaml::from_reader(reader)?;

        if index.len() != crate::COMPACT_CHUNK_COUNT {
            return Err(Error::InvalidDatabase(format!(
                "Invalid chunk count: {} != {}",
                index.len(),
                crate::COMPACT_CHUNK_COUNT
            )));
        }

        let pot = Arc::new(CookingPot::new()?);

        Ok(Self {
            path,
            index: index.into_boxed_slice(),
            pot,
        })
    }

    pub fn chunk_count(&self) -> usize {
        crate::COMPACT_CHUNK_COUNT
    }

    pub fn open_chunk(&self, chunk_id: usize) -> Result<Chunk, Error> {
        let chunk_path = self.path.join(format!("chunk_{}.rdb", chunk_id));
        if !chunk_path.exists() {
            return Err(Error::InvalidDatabase(format!(
                "Cannot find chunk_{}.rdb",
                chunk_id
            )));
        }
        Ok(Chunk::open(chunk_id, chunk_path)?)
    }

    pub fn open_filtered_chunk(
        &self,
        chunk_id: usize,
        filter: &Filter,
    ) -> Result<Option<FilteredChunk>, Error> {
        if self.index[chunk_id].can_skip(filter) {
            return Ok(None);
        }
        Ok(Some(
            self.open_chunk(chunk_id)?
                .filter(filter, Arc::clone(&self.pot)),
        ))
    }

    pub fn pot(&self) -> Arc<CookingPot> {
        Arc::clone(&self.pot)
    }
}

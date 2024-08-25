use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use crate::cook::CookingPot;

use super::{Error, Index};

/// Main database handle
pub struct Database {
    /// Path to the database folder (containg index.yaml, the chunks, etc)
    path: PathBuf,
    /// The index data. i-th element corresponds to the i-th chunk
    index: Box<[Index]>,
    /// The cooker, in case we need more information on crit
    pot: CookingPot,
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

        let pot = CookingPot::new()?;

        Ok(Self {
            path,
            index: index.into_boxed_slice(),
            pot,
        })
    }

    pub fn open_chunk(&self, chunk_id: usize) -> Result<BufReader<File>, Error> {
        let chunk_path = self.path.join(format!("chunk-{}.db", chunk_id));
        if !chunk_path.exists() {
            return Err(Error::InvalidDatabase(format!(
                "chunk-{}.db not found",
                chunk_id
            )));
        }
        Ok(BufReader::new(File::open(chunk_path)?))
    }
}

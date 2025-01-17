use std::fs::File;
use std::path::{Path, PathBuf};

use fs2::FileExt;
use log::{error, info};

use super::{meta, Chunk, Error, Filter, FilteredChunk, Index, TempResult};

/// Main database handle
pub struct Database {
    /// Lock file to prevent multiple instances of the app from accessing the database
    lock: File,
    /// Path to the database folder (containg index.yaml, the chunks, etc)
    path: PathBuf,
    /// The index data. i-th element corresponds to the i-th chunk
    index: Box<[Index]>,
}

impl Database {
    /// Open a database and loads the index data
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let lock_path = path.as_ref().join(".lock");
        info!("locking at {}", lock_path.display());
        if lock_path.exists() {
            return Err(Error::Locked);
        }
        let lock_file = File::create(&lock_path).map_err(|_| Error::Locked)?;
        lock_file.try_lock_exclusive().map_err(|_| Error::Locked)?;

        Self::open_locked(path, lock_file)
    }
    /// Open a database, assuming it's already locked
    pub fn open_locked(path: impl AsRef<Path>, lock_file: File) -> Result<Self, Error> {
        let path = path.as_ref().to_path_buf();
        info!("opening database at {}", path.display());

        info!("loading index.yaml");
        let index = meta::load_index(meta::index_path(&path))?;

        let db = Self {
            lock: lock_file,
            path,
            index: index.into_boxed_slice(),
        };

        if let Err(e) = db.delete_temporary() {
            error!("failed to delete temporary directory: {}", e.to_string());
        }

        info!("database opened");
        Ok(db)
    }

    pub fn chunk_count(&self) -> u32 {
        meta::compact().chunk_count()
    }

    pub fn open_chunk(&self, chunk_id: u32) -> Result<Chunk, Error> {
        let chunk_path = meta::compact_chunk_path(&self.path, chunk_id);
        if !chunk_path.exists() {
            return Err(Error::MissingChunk(chunk_id));
        }
        Chunk::open(chunk_id, chunk_path)
    }

    pub fn open_filtered_chunk(
        &self,
        chunk_id: u32,
        filter: &Filter,
    ) -> Result<Option<FilteredChunk>, Error> {
        if self.index[chunk_id as usize].can_skip(filter) {
            return Ok(None);
        }
        Ok(Some(
            self.open_chunk(chunk_id)?
                .filter(filter)
        ))
    }

    /// Delete temporary working directory in the database
    pub fn delete_temporary(&self) -> Result<(), Error> {
        let path = self.path.join("temp");
        if !path.exists() {
            return Ok(());
        }
        std::fs::remove_dir_all(path)?;
        Ok(())
    }

    /// Create a new location in the temporary working directory for saving results
    pub fn new_temporary(&self) -> Result<TempResult, Error> {
        let temp_path = self.path.join("temp");
        if !temp_path.exists() {
            std::fs::create_dir(&temp_path)?;
        }
        let mut id = rand::random::<u64>();
        // try 100 times
        for _ in 0..100 {
            let hex = format!("{:08x}", id);
            let path = temp_path.join(&hex);
            if !path.exists() && std::fs::create_dir(&path).is_ok() {
                info!("created temporary directory with id {}", hex);
                return Ok(TempResult::new(path));
            }
            id = id.wrapping_add(1);
        }
        Err(Error::TooManyTemporary)
    }

    pub fn close(&self) {
        info!("closing database");
        if let Err(e) = self.delete_temporary() {
            error!("failed to delete temporary directory: {}", e.to_string());
        }
        if let Err(e) = fs2::FileExt::unlock(&self.lock) {
            error!("failed to unlock database: {}", e.to_string());
        }
        let lock_path = self.path.join(".lock");
        if lock_path.exists() {
            if let Err(e) = std::fs::remove_file(lock_path) {
                error!("failed to remove lock file: {}", e.to_string());
            }
        }
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        self.close();
    }
}

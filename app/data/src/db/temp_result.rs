use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

use crate::recipe::RecipeId;

use super::Error;

/// Temporary results of a set of recipes
///
/// We use database's temp directory to store intermediate results,
/// since memory could be a constraint on lower-end devices.
///
/// Each temp result is stored at `<db>/temp/<id>/` where `id` is a random identifier.
/// Inside the directory, there are files named `0`, `1`, `2`, ... each containing
/// a list of recipe ids as usize.
///
/// The structure can be used to both read and write, but only one should be used.
#[derive(Debug, Clone)]
pub struct TempResult {
    /// Path to the temporary result directory (directory is under <db>/temp/)
    path: PathBuf,
    /// Total number of records stored
    size: usize,
}

impl TempResult {
    /// Create a new temporary result from the path for writing
    pub fn new(path: PathBuf) -> Self {
        Self { path, size: 0 }
    }

    /// Set the total number of records stored after writing
    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    /// Iterator through chunks of the recipes in this result
    pub fn iter(&self) -> TempResultIter {
        TempResultIter {
            path: self.path.clone(),
            next_file_id: 0,
            had_error: false,
        }
    }

    /// Open a writer to store results in the file_id
    pub fn writer(&self, file_id: usize) -> Result<TempResultWriter, Error> {
        let path = self.path.join(file_id.to_string());
        TempResultWriter::open(path)
    }

    /// Clear the temporary result directory
    pub fn clear(&mut self) -> Result<(), Error> {
        std::fs::remove_dir_all(&self.path)?;
        std::fs::create_dir(&self.path)?;
        self.size = 0;
        Ok(())
    }
}

pub struct TempResultIter {
    path: PathBuf,
    next_file_id: usize,
    had_error: bool,
}

impl Iterator for TempResultIter {
    type Item = Result<TempResultReader, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.had_error {
            return None;
        }
        loop {
            if self.next_file_id >= crate::COMPACT_CHUNK_COUNT {
                return None;
            }
            let path = self.path.join(self.next_file_id.to_string());
            if !path.exists() {
                self.next_file_id += 1;
                continue;
            }
            let reader = match TempResultReader::open(path) {
                Ok(reader) => reader,
                Err(e) => {
                    self.had_error = true;
                    return Some(Err(e));
                }
            };
            self.next_file_id += 1;
            return Some(Ok(reader));
        }
    }
}

pub struct TempResultWriter {
    path: PathBuf,
    writer: Option<BufWriter<File>>,
    size: usize,
}

impl TempResultWriter {
    pub fn open(path: PathBuf) -> Result<Self, Error> {
        Ok(Self {
            path,
            writer: None,
            size: 0,
        })
    }

    pub fn write(&mut self, id: RecipeId) -> Result<(), Error> {
        match self.writer.as_mut() {
            Some(writer) => {
                writer.write_all(&usize::from(id).to_le_bytes())?;
            }
            None => {
                let file = File::create(&self.path)?;
                let mut writer = BufWriter::new(file);
                writer.write_all(&usize::from(id).to_le_bytes())?;
                self.writer = Some(writer);
            }
        }
        self.size += 1;
        Ok(())
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

pub struct TempResultReader {
    reader: BufReader<File>,
    had_error: bool,
}
impl TempResultReader {
    pub fn open(path: PathBuf) -> Result<Self, Error> {
        let file = File::open(path)?;
        Ok(Self {
            reader: BufReader::new(file),
            had_error: false,
        })
    }
}
impl Iterator for TempResultReader {
    type Item = Result<RecipeId, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.had_error {
            return None;
        }
        let mut buf = [0u8; 8];
        match self.reader.read_exact(&mut buf) {
            Ok(()) => {
                let id = usize::from_le_bytes(buf);
                match RecipeId::new(id) {
                    Some(id) => Some(Ok(id)),
                    None => Some(Err(Error::InvalidRecipeId(id))),
                }
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    None
                } else {
                    self.had_error = true;
                    Some(Err(e.into()))
                }
            }
        }
    }
}

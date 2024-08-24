use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use positioned_io::{RandomAccessFile, ReadBytesAtExt};


/// The crit database

#[derive(Debug, Clone)]
pub struct CritDb {
    data: Arc<[u32; crate::CRIT_DB_U32_SIZE]>,
}

impl CritDb {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut data = Box::new([0; crate::CRIT_DB_U32_SIZE]);
        let mut reader = BufReader::new(File::open(path.as_ref())?);
        let mut buf = [0; 4];
        for record in data.iter_mut() {
            reader.read_exact(&mut buf)?;
            *record = u32::from_le_bytes(buf);
        }
        Ok(Self {
            data: Arc::from(data),
        })
    }

    pub fn get(&self, recipe_id: usize) -> bool {
        let u32_idx = recipe_id / 32;
        let bit_idx = recipe_id % 32;
        self.data[u32_idx] & (1 << bit_idx) != 0
    }
}

pub struct CritDbWriter {
    data: Box<[u32; crate::CRIT_DB_U32_SIZE]>,
}

impl CritDbWriter {
    pub fn new() -> Self {
        CritDbWriter {
            data: Box::new([0; crate::CRIT_DB_U32_SIZE]),
        }
    }

    pub fn set(&mut self, recipe_id: usize, crit_rng_hp: bool) {
        // recipe_id = 32 * u32_idx + bit_idx
        let u32_idx = recipe_id / 32;
        let bit_idx = recipe_id % 32;
        let record = self.data[u32_idx];
        if crit_rng_hp {
            self.data[u32_idx] = record | (1 << bit_idx);
        } else {
            self.data[u32_idx] = record & !(1 << bit_idx);
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut file = BufWriter::new(File::create(path.as_ref())?);
        for &record in self.data.iter() {
            file.write_all(&record.to_le_bytes())?;
        }
        Ok(())
    }
}

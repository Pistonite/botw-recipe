use std::{fs::File, io::{BufReader, Read}, path::Path};

use crate::wmc::WeaponModifier;

use super::Error;


#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Record(u16);

impl From<u16> for Record {
    fn from(value: u16) -> Self {
        Record(value)
    }
}

impl Record {
    #[inline]
    pub fn raw(self) -> u16 {
        self.0
    }
    #[inline]
    pub fn value(self) -> i32 {
        (self.0 >> 9).into()
    }
    #[inline]
    pub fn modifier(self) -> u32 {
        ((self.0 as u32) & 0x1FF).into()
    }
}

/// Sequentially read data from a chunk
pub struct ChunkReader {
    reader: BufReader<File>,
    next: usize,
    total: usize,
}

impl ChunkReader {
    pub fn open<P: AsRef<Path>>(chunk_id: usize, path: P) -> Result<Self, Error> {
        let file = File::open(path.as_ref())?;
        let total = if chunk_id == crate::COMPACT_CHUNK_COUNT - 1 {
            crate::COMPACT_LAST_CHUNK_SIZE
        } else {
            crate::COMPACT_CHUNK_SIZE
        };
        let file_size = file.metadata()?.len() as usize;
        if file_size != total * 2 {
            return Err(Error::InvalidChunkSize(file_size, total * 2));
        }
        Ok(Self {
            reader: BufReader::new(file),
            next: 0,
            total,
        })
    }
}

impl Iterator for ChunkReader {
    type Item = Result<Record, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.total {
            return None;
        }
        let mut buf = [0; 2];
        match self.reader.read_exact(&mut buf) {
            Err(e) => Some(Err(e.into())),
            Ok(_) => {
                self.next += 1;
                Some(Ok(Record(u16::from_le_bytes(buf))))
            }
        }
    }
}

use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::sync::LazyLock;

use log::info;
use rdata::db::Database;
use rdata::recipe::RecipeId;

use crate::error::Error;

pub fn create_database() -> LazyLock<Result<Database, Error>> {
    
    LazyLock::new(|| {
        let path = std::env::var("BOTWRDB_PATH").unwrap_or("database/".to_string());
        info!("opening database from {}", path);
        match Database::open(&path) {
            Ok(db) => Ok(db),
            Err(e) => {
                info!("failed to open database: {}", e);
                Err(Error::DatabaseError(e))
            }
        }
    })
}


pub fn save_search_result(result: &[RecipeId]) -> Result<(), Error> {
    info!("saving search result to search_result.bin");
    let mut writer = BufWriter::new(File::create("search_result.bin")?);
    for id in result {
            writer.write_all(&usize::from(*id).to_le_bytes())?;
    }
    info!("search result saved");
    Ok(())
}

pub struct SearchResultReader {
    reader: BufReader<File>,
}

impl Iterator for SearchResultReader {
    type Item = Result<RecipeId, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0u8; 8];
        match self.reader.read_exact(&mut buf) {
            Ok(()) => {
                let id = usize::from_le_bytes(buf);
                match RecipeId::new(id) {
                    Some(id) => Some(Ok(id)),
                    None => Some(Err(Error::InvalidSearchResult)),
                }
            }
            Err(_) => None,
        }
    }
}

pub fn open_search_result() -> Result<SearchResultReader, Error> {
    info!("opening search result from search_result.bin");
    let path = Path::new("search_result.bin");
    if !path.exists() {
        return Err(Error::MissingSearchResult);
    }
    let reader = BufReader::new(File::open("search_result.bin")?);
    Ok(SearchResultReader { reader })
}
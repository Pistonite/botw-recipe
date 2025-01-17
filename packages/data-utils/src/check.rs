//! Utils for testing the correctness of the cooking simulator
//! by checking the raw database dump against a known good dump
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::mpsc;

use botw_recipe_cook::CookData;
use botw_recipe_wmcdb::meta;

use crate::{util, Error};

/// Check if RawDB in the database directory is valid using a thread pool
pub fn check_raw_db(path: &Path) -> anyhow::Result<()> {
    let meta = meta::raw();
    let chunk_count = meta.chunk_count();
    let progress = spp::printer(chunk_count as usize, format!("Checking {}", path.display()));
    let mut errors = Vec::new();
    let pool = crate::thread_pool();
    let (send, recv) = mpsc::channel();
    for chunk_id in 0..chunk_count {
        let chunk_size = meta.chunk_size(chunk_id);
        let chunk_size_bytes = meta.chunk_size_bytes(chunk_id);
        let chunk_path = meta::raw_chunk_path(path, chunk_id);
        let send = send.clone();
        pool.execute(move || {
            let result = check_raw_chunk(chunk_size, chunk_size_bytes, &chunk_path);
            let _ = send.send((chunk_id, result));
        });
    }
    drop(send);
    for (i, (chunk_id, result)) in recv.into_iter().enumerate() {
        progress.print(i, format!("Finished Chunk {}", chunk_id));
        if let Err(e) = result {
            errors.push((chunk_id, e));
        }
    }
    progress.done();
    util::check_errors(&errors)
}

/// Check if every record in a raw chunk is valid
///
/// Returns the first invalid record if found
pub fn check_raw_chunk(
    records: usize,
    size_bytes: usize,
    path: &Path,
) -> Result<(), Error> {
    let file_size = path.metadata()?.len() as usize;
    if file_size != size_bytes {
        return Err(Error::InvalidSize(file_size, size_bytes));
    }
    let mut reader = BufReader::new(File::open(path)?);

    for i in 0..records {
        let data = CookData::read_from(&mut reader)?;
        let invalid_reason = data.is_normal();
        if let Some(reason) = invalid_reason {
            return Err(Error::InvalidRecord(i, reason, data));
        }
    }
    Ok(())
}

/// Check if 2 RawDBs are the same
pub fn compare_raw_db(path_a: &Path, path_b: &Path) -> anyhow::Result<()> {
    let meta = meta::raw();
    let chunk_count = meta.chunk_count();
    let progress = spp::printer(
        chunk_count as usize,
        format!("Comparing {} and {}", path_a.display(), path_b.display()),
    );
    let mut errors = Vec::new();
    let pool = crate::thread_pool();
    let (send, recv) = mpsc::channel();
    for chunk_id in 0..chunk_count {
        let chunk_size = meta.chunk_size(chunk_id);
        let chunk_path_a = meta::raw_chunk_path(path_a, chunk_id);
        let chunk_path_b = meta::raw_chunk_path(path_b, chunk_id);
        let send = send.clone();
        pool.execute(move || {
            let result = compare_raw_chunks(chunk_size, &chunk_path_a, &chunk_path_b);
            let _ = send.send((chunk_id, result));
        });
    }
    drop(send);
    let mut matched_count = 0;
    for (i, (chunk_id, result)) in recv.into_iter().enumerate() {
        progress.print(i, format!("Chunk {}", chunk_id));
        match result {
            Ok(count) => matched_count += count,
            Err(e) => {
                errors.push((chunk_id, e));
            }
        }
    }
    progress.done();
    util::check_errors(&errors)?;

    println!("Matched {} records", matched_count);
    Ok(())
}

/// Compare if 2 raw chunks are the same
///
/// Returns the number of records that match and the first mismatch if found
pub fn compare_raw_chunks(records: usize, path_a: &Path, path_b: &Path) -> Result<usize, Error> {
    if !path_a.exists() || !path_b.exists() {
        return Err(Error::NotFound);
    }
    let mut reader_a = BufReader::new(File::open(path_a)?);
    let mut reader_b = BufReader::new(File::open(path_b)?);

    let mut mismatch = None;
    let mut matched_count = 0;

    for i in 0..records {
        let data_a = CookData::read_from(&mut reader_a)?;
        let data_b = CookData::read_from(&mut reader_b)?;

        if data_a != data_b {
            if mismatch.is_none() {
                mismatch = Some((i, data_a, data_b));
            }
        } else {
            matched_count += 1;
        }
    }
    if let Some((i, data_a, data_b)) = mismatch {
        return Err(Error::Mismatch(i, data_a, data_b, matched_count));
    }

    Ok(matched_count)
}

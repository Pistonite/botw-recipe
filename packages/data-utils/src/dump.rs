//! Util for dumping the database using the cooking simulator
//! in both compact and raw formats

use std::fs::{self, File};
use std::io::BufWriter;
use std::path::Path;
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};

use anyhow::bail;

use botw_recipe::cook::{CookData, CookingPot};
use botw_recipe::db::{Index, IndexBuilder, Record};
use botw_recipe::fsdb;

use crate::util;

/// Dump the RawDB to the given path
pub fn dump_raw_db(path: &Path) -> anyhow::Result<()> {
    let start_time = Instant::now();
    let meta = fsdb::meta::raw_v2();
    let chunk_count = meta.chunk_count();

    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    let mut progress = spp::printer(chunk_count as usize, format!("Dumping RawDB to {}", path.display()));
    progress.set_throttle_duration(Duration::from_secs(1));
    let pool = crate::thread_pool();
    let pot = Arc::new(CookingPot::new()?);
    let (send, recv) = mpsc::channel();

    for chunk_id in 0..chunk_count as u32{
        let send = send.clone();
        let pot = Arc::clone(&pot);
        let (start, end) = meta.record_range(chunk_id);
        let chunk_path = fsdb::meta::raw_chunk_path(path, chunk_id);
        pool.execute(
            move || match dump_raw_chunk(&pot, &chunk_path, start, end) {
                Ok(_) => {
                    let _ = send.send((chunk_id, Ok(())));
                }
                Err(e) => {
                    let _ = send.send((chunk_id, Err(e.to_string())));
                }
            },
        );
    }
    drop(send);
    let mut errors = vec![];
    for (i, (chunk_id, result)) in recv.into_iter().enumerate() {
        progress.print(i, format!("Finished Chunk {}", chunk_id));
        if let Err(e) = result {
            errors.push((chunk_id, e));
        }
    }
    progress.done();
    util::check_errors(&errors)?;

    println!("Done in {:.2}s", start_time.elapsed().as_secs_f32());

    Ok(())
}

/// Cook recipes from start to end IDs and write them to a RawDB chunk file
pub fn dump_raw_chunk(
    pot: &CookingPot,
    chunk_path: &Path,
    start: u64,
    end: u64,
) -> anyhow::Result<()> {
    let mut writer = BufWriter::new(File::create(chunk_path)?);
    for id in start..end {
        let data = if id == 0 {
            CookData::invalid()
        } else {
            pot.cook_id(id)?.data
        };
        data.write_to(&mut writer)?;
    }

    Ok(())
}

/// Dump the CompactDB to the given path
pub fn dump_compact_db(path: &Path) -> anyhow::Result<()> {
    let start_time = Instant::now();
    let meta = fsdb::meta::compact_v2();
    let chunk_count = meta.chunk_count();

    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    let mut progress = spp::printer(
        chunk_count as usize,
        format!("Dumping CompactDB to {}", path.display()),
    );
    progress.set_throttle_duration(Duration::from_secs(1));
    let pool = crate::thread_pool();
    let pot = Arc::new(CookingPot::new()?);
    let (send, recv) = mpsc::channel();

    for chunk_id in 0..chunk_count as u32 {
        let send = send.clone();
        let pot = Arc::clone(&pot);
        let (start, end) = meta.record_range(chunk_id);
        let chunk_path = fsdb::meta::compact_chunk_path(path, chunk_id);
        pool.execute(
            move || match dump_compact_chunk(&pot, chunk_id, &chunk_path, start, end) {
                Ok(index) => {
                    let _ = send.send((chunk_id, Ok(index)));
                }
                Err(e) => {
                    let _ = send.send((chunk_id, Err(e.to_string())));
                }
            },
        );
    }
    drop(send);
    let mut index_vec = Vec::new();
    let mut errors = vec![];
    for (i, (chunk_id, result)) in recv.into_iter().enumerate() {
        progress.print(i, format!("Finished Chunk {}", chunk_id));
        match result {
            Ok(index) => {
                index_vec.push(index);
            }
            Err(e) => {
                errors.push((chunk_id, e));
            }
        }
    }
    progress.done();
    util::check_errors(&errors)?;

    index_vec.sort_unstable_by_key(|index| index.chunk);

    for (i, index) in index_vec.iter().enumerate() {
        if index.chunk != i {
            bail!("Index for chunk {} is out of order", index.chunk);
        }
    }

    fsdb::save_index(fsdb::index_path(path), &index_vec)?;

    println!("Done in {:.2}s", start_time.elapsed().as_secs_f32());

    Ok(())
}

/// Cook recipes from start to end IDs and write them to a CompactDB chunk file and index
pub fn dump_compact_chunk(
    pot: &CookingPot,
    chunk_id: u32,
    chunk_path: &Path,
    start: u64,
    end: u64,
) -> anyhow::Result<Index> {
    let mut index = IndexBuilder::new(chunk_id as usize);
    let mut writer = BufWriter::new(File::create(chunk_path)?);
    for id in start..end {
        let (data, crit_rng_hp) = if id == 0 {
            (CookData::invalid(), false)
        } else {
            let result = pot.cook_id(id)?;
            (result.data, result.crit_rng_hp)
        };

        let record = Record::from_data(&data, crit_rng_hp);
        record.write(&mut writer)?;
        index.update(&data, crit_rng_hp);
    }

    Ok(index.build())
}

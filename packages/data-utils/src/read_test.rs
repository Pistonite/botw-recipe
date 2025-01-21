//! Testing reading from CompactDB, and compare the result
//! from the simulator
//!
//! This is used to verify CompactDB is correctly generated

use std::path::Path;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use anyhow::bail;
use botw_recipe_wmcdb::{Chunk, Database};

use crate::util;

/// Read the CompactDB at the directory using the database API,
/// and verify the results are the same as the simulator output
pub fn test_read_db(path: &Path) -> anyhow::Result<()> {
    let start_time = Instant::now();
    let database = Database::open(path)?;
    let chunk_count = database.chunk_count();
    let mut progress = spp::printer(
        chunk_count as usize,
        format!("Read-testing CompactDB at {}", path.display()),
    );
    progress.set_throttle_duration(Duration::from_secs(1));

    let pool = crate::thread_pool();
    let (send, recv) = mpsc::channel();

    for chunk_id in 0..chunk_count {
        let send = send.clone();
        let chunk = database.open_chunk(chunk_id)?;

        pool.execute(move || match test_read_chunk(chunk) {
            Ok(()) => {
                let _ = send.send((chunk_id, Ok(())));
            }
            Err(e) => {
                let _ = send.send((chunk_id, Err(e.to_string())));
            }
        });
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

pub fn test_read_chunk(chunk: Chunk) -> anyhow::Result<()> {
    for record in chunk {
        let record = record?;
        let cooked = botw_recipe_cook::cook_id_unchecked(record.recipe_id);
        let data = cooked.get_wmc_data();
        let expected_value = data.price & 0x1FF;
        if expected_value != record.record.modifier() as i32 {
            bail!(
                "Recipe {}, Mismatched modifier/price: expected {}, got {}",
                u64::from(record.recipe_id),
                expected_value,
                record.record.modifier()
            );
        }
        if data.hp != record.record.value() {
            bail!(
                "Recipe {}, Mismatched value/hp: expected {}, got {}",
                u64::from(record.recipe_id),
                data.hp,
                record.record.value()
            );
        }
    }

    Ok(())
}

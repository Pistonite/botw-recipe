use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use clap::Parser;
use rdata::cook::{CookData, CookEffect, CookingPot};
use rdata::db::{Index, IndexBuilder};
use threadpool::ThreadPool;

#[derive(Parser, Clone)]
pub struct Cli {
    /// Keep existing files instead of overwriting them
    #[clap(short, long)]
    pub keep: bool,

    /// Chunk id to start from
    #[clap(short, long, default_value = "0")]
    pub start: usize,

    /// How many chunks to dump
    #[clap(short = 'n', long)]
    pub count: Option<usize>,

    /// Dump compact DB instead of raw DB
    #[clap(short = 'C', long)]
    pub compact: bool,
}

fn main() {
    if let Err(e) = dump(Cli::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn chunk_meta(cli: &Cli) -> (usize, usize, usize) {
    if cli.compact {
        (
            rdata::COMPACT_CHUNK_SIZE,
            rdata::COMPACT_CHUNK_COUNT,
            rdata::COMPACT_LAST_CHUNK_SIZE,
        )
    } else {
        (
            rdata::CHUNK_SIZE,
            rdata::CHUNK_COUNT,
            rdata::LAST_CHUNK_SIZE,
        )
    }
}

fn data_folder(cli: &Cli) -> &'static str {
    if cli.compact {
        "./compact"
    } else {
        "./data"
    }
}

fn chunk_path(cli: &Cli, base: &Path, id: usize) -> PathBuf {
    if cli.compact {
        base.join(format!("chunk_{id}.rdb"))
    } else {
        base.join(format!("chunk_{id}.rawdat"))
    }
}

fn dump(cli: Cli) -> anyhow::Result<()> {
    let data_path = Path::new(data_folder(&cli));
    if !data_path.exists() {
        std::fs::create_dir_all(data_path)?;
    }

    let (chunk_size, chunk_count, _) = chunk_meta(&cli);

    let total = if cli.compact {
        if cli.count.is_some() {
            return Err(anyhow::anyhow!("count is not supported for compact mode"));
        }
        chunk_count
    } else {
        let total = cli.count.unwrap_or(chunk_count) - cli.start;
        if total > chunk_count {
            return Err(anyhow::anyhow!("count is too large"));
        }
        total
    };

    let start_time = Instant::now();
    let num_workers = total.min(num_cpus::get());
    println!("using {} threads", num_workers);
    let pool = ThreadPool::new(num_workers);

    let pot = Arc::new(CookingPot::new()?);
    let aborted = Arc::new(AtomicBool::new(false));
    let (send, recv) = mpsc::channel();

    for chunk_id in cli.start..cli.start + total {
        let send = send.clone();
        let pot = Arc::clone(&pot);
        let aborted = Arc::clone(&aborted);
        let cli = cli.clone();
        pool.execute(move || {
            if aborted.load(std::sync::atomic::Ordering::Relaxed) {
                return;
            }
            match dump_chunk(&pot, chunk_id, &cli) {
                Ok((did_work, index)) => {
                    let _ = send.send((chunk_id, None, did_work, index));
                }
                Err(e) => {
                    let _ = send.send((chunk_id, Some(e.to_string()), false, None));
                }
            }
        });
    }

    drop(send);

    let total_count = Arc::new(AtomicUsize::new(0));
    let dumped_count = Arc::new(AtomicUsize::new(0));
    let counting_thread = {
        let total_count = total_count.clone();
        let dumped_count = dumped_count.clone();
        let aborted = aborted.clone();
        thread::spawn(move || loop {
            let total_count = total_count.load(std::sync::atomic::Ordering::Relaxed);
            let elapsed = start_time.elapsed().as_secs_f32();

            let dumped_count = dumped_count.load(std::sync::atomic::Ordering::Relaxed);
            let eta_str = if dumped_count == 0 {
                "".to_string()
            } else {
                let speed = dumped_count as f32 / elapsed;
                let remaining = total - total_count;
                let recipe_count = dumped_count * chunk_size;
                let recipe_per_sec = (recipe_count as f32 / elapsed) as usize;
                let remaining_seconds = remaining as f32 / speed;
                format!(
                    "({:.02} records/s | ETA {:.02}s)",
                    recipe_per_sec, remaining_seconds
                )
            };

            if aborted.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
            if total_count >= total {
                println!("chunks finished {}/{} {}", total_count, total, eta_str);
                break;
            }
            if elapsed > 1.0 {
                println!("chunks finished {}/{} {}", total_count, total, eta_str);
                thread::sleep(std::time::Duration::from_secs(1));
            }
        })
    };

    let mut success = true;
    let mut index_vec = Vec::new();
    for (chunk_id, error, did_work, index) in recv {
        total_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if did_work {
            dumped_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        if let Some(error) = error {
            println!("chunk {} failed: {}", chunk_id, error);
            aborted.store(true, std::sync::atomic::Ordering::Relaxed);
            success = false;
            break;
        }
        if let Some(index) = index {
            index_vec.push(index);
        }
    }
    pool.join();
    counting_thread
        .join()
        .map_err(|_| anyhow::anyhow!("counting thread panicked"))?;

    if !success {
        return Err(anyhow::anyhow!("dumping failed"));
    }

    if cli.compact {
        println!("checking index");
        index_vec.sort_unstable_by_key(|index| index.chunk);

        for (i, index) in index_vec.iter().enumerate() {
            if index.chunk != i {
                return Err(anyhow::anyhow!(
                    "index chunk {} is out of order",
                    index.chunk
                ));
            }
        }

        println!("saving index");
        let writer = BufWriter::new(File::create(
            Path::new(data_folder(&cli)).join("index.yaml"),
        )?);
        serde_yaml_ng::to_writer(writer, &index_vec)?;
    }

    println!("done in {:.2}s", start_time.elapsed().as_secs_f32());

    Ok(())
}

fn dump_chunk(
    pot: &CookingPot,
    chunk_id: usize,
    cli: &Cli,
) -> anyhow::Result<(bool, Option<Index>)> {
    let data_path = chunk_path(cli, Path::new(data_folder(cli)), chunk_id);
    if cli.keep && data_path.exists() {
        println!("chunk {} already exists, skipping", chunk_id);
        return Ok((false, None));
    }

    let mut writer = BufWriter::new(File::create(data_path)?);

    let (chunk_size, _, _) = chunk_meta(cli);

    let chunk_start = chunk_id * chunk_size;
    let chunk_end = rdata::NUM_TOTAL_RECORDS.min(chunk_start + chunk_size);

    let result = if cli.compact {
        let mut index = IndexBuilder::new(chunk_id);
        cook_and_write_chunk_compact(pot, chunk_start, chunk_end, &mut writer, &mut index)?;
        (true, Some(index.build()))
    } else {
        cook_and_write_chunk(pot, chunk_start, chunk_end, &mut writer)?;
        (true, None)
    };

    Ok(result)
}

fn cook_and_write_chunk(
    pot: &CookingPot,
    start: usize,
    end: usize,
    writer: &mut BufWriter<File>,
) -> anyhow::Result<()> {
    for id in start..end {
        let data = if id == 0 {
            CookData::invalid()
        } else {
            pot.cook_id(id)?.data
        };
        data.write_to(writer)?;
    }

    Ok(())
}

fn cook_and_write_chunk_compact(
    pot: &CookingPot,
    start: usize,
    end: usize,
    writer: &mut BufWriter<File>,
    index: &mut IndexBuilder,
) -> anyhow::Result<()> {
    for id in start..end {
        let (data, crit_rng_hp) = if id == 0 {
            (CookData::invalid(), false)
        } else {
            let result = pot.cook_id(id)?;
            (result.data, result.crit_rng_hp)
        };
        let mut hp = data.health_recover;
        if data.crit_chance >= 100 && !crit_rng_hp {
            // guaranteed crit but no heart rng crit, which means guaranteed heart crit
            if data.effect_id == CookEffect::LifeMaxUp.game_repr_f32() {
                // hearty adds 4
                // technically this should go out of max, because it's 108 + 4
                // (max is 108 but game doesn't check the cap when crit)
                hp += 4;
            } else {
                hp = (hp + 12).min(120);
            }
        }
        let price = data.sell_price;
        // hhhh hhhp pppp pppp
        let low = (price & 0xFF) as u8;
        let high = (hp << 1) as u8 | ((price >> 8) & 0x01) as u8;
        writer.write_all(&[low, high])?;
        index.update(&data, crit_rng_hp);
    }

    Ok(())
}

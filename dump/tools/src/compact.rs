use std::fs::File;
use std::io::{BufReader, BufWriter, Read};
use std::sync::atomic::AtomicUsize;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Instant;

use rdata::db::{CritDb, CritDbWriter, Index};
use threadpool::ThreadPool;

fn main() -> anyhow::Result<()> {
    let start_time = Instant::now();

    // compact_critdb()?;
    compute_indices()?;

    let elapsed = start_time.elapsed();
    println!("done in {:.02}s", elapsed.as_secs_f32());

    Ok(())
}

fn compact_critdb() -> anyhow::Result<()> {
    let threads = num_cpus::get();
    let pool = ThreadPool::new(threads);
    println!("packing crit.db using {} threads", threads);
    let (send, recv) = mpsc::channel::<bool>();
    let critdb = Arc::new(Mutex::new(CritDbWriter::new()));
    for chunk_id in 0..rdata::COMPACT_CHUNK_COUNT {
        let crit_db = Arc::clone(&critdb);
        let send = send.clone();
        pool.execute(move || {
            if let Err(e) = compact_critdb_chunk(chunk_id, &crit_db) {
                eprintln!("error compacting chunk {}: {:?}", chunk_id, e);
                send.send(false).unwrap();
            } else {
                send.send(true).unwrap();
            }
        });
    }
    drop(send);
    let count = Arc::new(AtomicUsize::new(0));
    {
        let count = Arc::clone(&count);
        pool.execute(move || {
            loop {
                let c = count.load(std::sync::atomic::Ordering::Relaxed);
                println!("compacted {}/{} chunks", c, rdata::COMPACT_CHUNK_COUNT);
                if c == rdata::COMPACT_CHUNK_COUNT {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });
    }

    let mut success = true;
    for result in recv {
        count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if !result {
            success = false;
        }
    }
    pool.join();

    if !success {
        return Err(anyhow::anyhow!("compacting crit.db failed"));
    }

    println!("saving compacted crit.db");
    critdb.lock().unwrap().save("../emulate/compact/crit.db")?;

    Ok(())
}

fn compact_critdb_chunk(chunk_id: usize, writer: &Arc<Mutex<CritDbWriter>>) -> anyhow::Result<()> {
    let len = if chunk_id == rdata::COMPACT_CHUNK_COUNT - 1 {
        rdata::COMPACT_LAST_CHUNK_SIZE
    } else {
        rdata::COMPACT_CHUNK_SIZE
    };
    let recipe_start = chunk_id * rdata::COMPACT_CHUNK_SIZE;
    let path = format!("../emulate/compact/crit_{chunk_id}.rawdat");
    let mut reader = BufReader::new(File::open(&path)?);
    let mut buf = Vec::with_capacity(len);
    let read = reader.read_to_end(&mut buf)?;
    if read != len {
        return Err(anyhow::anyhow!("chunk {} read {} bytes, expected {}", chunk_id, read, len));
    }
    let mut writer = writer.lock().unwrap();
    for i in 0..len {
        let recipe_id = recipe_start + i;
        let crit_rng_hp = buf[i] != 0;
        writer.set(recipe_id, crit_rng_hp);
    }

    std::fs::remove_file(&path)?;

    Ok(())

}

fn compute_indices() -> anyhow::Result<()> {
    let crit_db = CritDb::open("../emulate/compact/crit.db")?;
    let threads = num_cpus::get();
    let pool = ThreadPool::new(threads);
    println!("computing indices using {} threads", threads);
    let (send, recv) = mpsc::channel::<Option<Index>>();
    for chunk_id in 0..rdata::COMPACT_CHUNK_COUNT {
        let crit_db = crit_db.clone();
        let send = send.clone();
        pool.execute(move || {
            match compute_index_for_chunk(chunk_id, &crit_db) {
                Ok(index) => send.send(Some(index)).unwrap(),
                Err(e) => {
                    eprintln!("error computing index for chunk {}: {:?}", chunk_id, e);
                    send.send(None).unwrap();
                }
            }
        });
    }
    drop(send);
    let count = Arc::new(AtomicUsize::new(0));
    let same_count = Arc::new(AtomicUsize::new(0));
    {
        let count = Arc::clone(&count);
        let same_count = Arc::clone(&same_count);
        pool.execute(move || {
            loop {
                let c = count.load(std::sync::atomic::Ordering::Relaxed);
                let s = same_count.load(std::sync::atomic::Ordering::Relaxed);
                println!("computed index for {}/{} chunks (optimized {} chunks)", c, rdata::COMPACT_CHUNK_COUNT, s);
                if c == rdata::COMPACT_CHUNK_COUNT {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });
    }

    let mut success = true;
    let mut indices = Vec::with_capacity(rdata::COMPACT_CHUNK_COUNT);
    for result in recv {
        count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        match result {
            Some(index) => {
                if index.is_all_same() {
                    same_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
                indices.push(index);
            },
            None => success = false,
        }
    }
    pool.join();

    if !success {
        return Err(anyhow::anyhow!("computing index failed"));
    }
    println!("checking index");
    indices.sort_unstable_by_key(|index| index.chunk);

    for (i, index) in indices.iter().enumerate() {
        if index.chunk != i {
            return Err(anyhow::anyhow!("index chunk {} is out of order", index.chunk));
        }
    }

    println!("saving index");
    let writer = BufWriter::new(File::create("../emulate/compact/index.yaml")?);
    serde_yaml::to_writer(writer, &indices)?;

    Ok(())
}

fn compute_index_for_chunk(chunk_id: usize, crit_db: &CritDb) -> anyhow::Result<Index> {
    let path = format!("../emulate/compact/chunk_{chunk_id}.rdb");
    let index = Index::compute(crit_db, chunk_id, &path)?;
    if index.is_all_same() {
        std::fs::remove_file(&path)?;
    }
    Ok(index)
}

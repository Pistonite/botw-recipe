use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use rdata::{
    cook::{CookEffect, CookingPot},
    db::{Chunk, Database},
};
use threadpool::ThreadPool;

fn main() {
    if let Err(e) = test_read() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn test_read() -> anyhow::Result<()> {
    let start_time = Instant::now();
    let database = Database::open("./compact")?;

    let num_workers = num_cpus::get();
    println!("using {} threads", num_workers);
    let pool = ThreadPool::new(num_workers);

    let pot = database.pot();
    let aborted = Arc::new(AtomicBool::new(false));
    let (send, recv) = mpsc::channel();

    for chunk_id in 0..database.chunk_count() {
        let send = send.clone();
        let chunk = database.open_chunk(chunk_id)?;
        let aborted = Arc::clone(&aborted);
        let pot = Arc::clone(&pot);

        pool.execute(move || {
            if aborted.load(std::sync::atomic::Ordering::Relaxed) {
                return;
            }
            match test_read_chunk(chunk, &pot) {
                Ok(()) => {
                    send.send(Ok(chunk_id)).unwrap();
                }
                Err(e) => {
                    send.send(Err((chunk_id, e.to_string()))).unwrap();
                }
            }
        });
    }
    drop(send);

    let total = database.chunk_count();
    let count = Arc::new(AtomicUsize::new(0));
    let counting_thread = {
        let count = count.clone();
        let aborted = aborted.clone();
        thread::spawn(move || loop {
            let count = count.load(std::sync::atomic::Ordering::Relaxed);
            let elapsed = start_time.elapsed().as_secs_f32();

            let eta_str = if count == 0 {
                "".to_string()
            } else {
                let speed = count as f32 / elapsed;
                let remaining = total - count;
                let chunks_per_sec = (count as f32 / elapsed) as usize;
                let remaining_seconds = remaining as f32 / speed;
                format!(
                    "({:.02} chunks/s | ETA {:.02}s)",
                    chunks_per_sec, remaining_seconds
                )
            };

            if aborted.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
            if count >= total {
                println!("chunks finished {}/{} {}", count, total, eta_str);
                break;
            }
            if elapsed > 1.0 {
                println!("chunks finished {}/{} {}", count, total, eta_str);
                thread::sleep(std::time::Duration::from_secs(1));
            }
        })
    };

    let mut success = true;
    for result in recv {
        count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        match result {
            Ok(_) => {}
            Err((chunk_id, error)) => {
                println!("chunk {} failed: {}", chunk_id, error);
                aborted.store(true, std::sync::atomic::Ordering::Relaxed);
                success = false;
            }
        }
    }
    pool.join();
    counting_thread
        .join()
        .map_err(|_| anyhow::anyhow!("counting thread panicked"))?;

    if !success {
        return Err(anyhow::anyhow!("testing read failed"));
    }

    println!("done in {:.2}s", start_time.elapsed().as_secs_f32());

    Ok(())
}

fn test_read_chunk(chunk: Chunk, pot: &CookingPot) -> anyhow::Result<()> {
    for record in chunk {
        let record = record?;
        let cooked = pot.cook_inputs(record.recipe_id)?;
        let expected_value = cooked.data.sell_price & 0x1FF;
        if expected_value != record.record.modifier() as i32 {
            Err(anyhow::anyhow!(
                "Recipe {}, Mismatched modifier: expected {}, got {}",
                usize::from(record.recipe_id),
                expected_value,
                record.record.modifier()
            ))?;
        }
        let expected_hp = if !cooked.crit_rng_hp && cooked.data.crit_chance >= 100 {
            if cooked.data.effect_id == CookEffect::LifeMaxUp.game_repr_f32() {
                cooked.data.health_recover + 4
            } else {
                (cooked.data.health_recover + 12).min(120)
            }
        } else {
            cooked.data.health_recover
        };
        if expected_hp != record.record.value() {
            Err(anyhow::anyhow!(
                "Recipe {}, Mismatched value: expected {}, got {}",
                usize::from(record.recipe_id),
                expected_hp,
                record.record.value()
            ))?;
        }
    }

    Ok(())
}

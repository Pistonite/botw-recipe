use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::atomic::AtomicUsize;
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Instant;

use clap::Parser;
use rdata::cook::CookData;
use rcook::CookingPot;

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
}

fn main() {
    dump(Cli::parse());
}

fn dump(cli: Cli) {
    let data_path = Path::new("./data");
    if !data_path.exists() {
        std::fs::create_dir_all(data_path).unwrap();
    }

    let total = cli.count.unwrap_or(rdata::CHUNK_COUNT) - cli.start;
    if total > rdata::CHUNK_COUNT {
        eprintln!("total chunks must be less than or equal to {}", rdata::CHUNK_COUNT);
        std::process::exit(1);
    }

    let start_time = Instant::now();
    let num_workers = total.min(num_cpus::get());
    println!("using {} threads", num_workers);

    let (output_send, output_recv) = mpsc::channel();
    let mut input_sends = Vec::with_capacity(num_workers);
    let mut handles = Vec::with_capacity(num_workers);
    for i in 0..num_workers {
        let send = output_send.clone();
        let (input_send, recv) = mpsc::channel();
        let cli2 = cli.clone();
        let handle = start_worker_thread(i, send, recv, cli2);
        // send the first job
        input_send.send(i+cli.start).unwrap();
        input_sends.push(input_send);
        handles.push(handle);
    }
    drop(output_send);

    let count = Arc::new(AtomicUsize::new(0));
    let work_count = Arc::new(AtomicUsize::new(0));
    let counting_thread = {
        let count = count.clone();
        let work_count = work_count.clone();
        thread::spawn(move || {
            loop {
                let mut recipe_per_sec = 0;
                let real_count = count.load(std::sync::atomic::Ordering::Relaxed);
                let c = work_count.load(std::sync::atomic::Ordering::Relaxed);
                let elapsed = start_time.elapsed().as_secs_f32();
                let remaining_seconds = if c == 0 {
                    0.0
                } else {
                    let speed = c as f32 / elapsed;
                    let remaining = total - c;
                    let recipe_count = c * rdata::CHUNK_SIZE;
                    recipe_per_sec = (recipe_count as f32 / elapsed) as usize;
                    remaining as f32 / speed
                };
                if real_count >= total {
                    println!("chunks finished {}/{} ({:.02} records/s | ETA {:.02}s)", real_count, total, recipe_per_sec, remaining_seconds);
                    break;
                }
                if elapsed > 2.0 {
                    println!("chunks finished {}/{} ({:.02} records/s | ETA {:.02}s)", real_count, total, recipe_per_sec, remaining_seconds);
                    thread::sleep(std::time::Duration::from_secs(2));
                }
            }
        })
    };
    let mut next = num_workers + cli.start;
    for (who_finished, did_work) in output_recv {
        count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if did_work {
            work_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        if next < total {
            input_sends[who_finished].send(next).unwrap();
        } else {
            let c = count.load(std::sync::atomic::Ordering::Relaxed);
            if c >= total {
                break;
            }
        }
        next += 1;
    }
    for send in input_sends {
        drop(send);
    }
    counting_thread.join().unwrap();
    for handle in handles {
        handle.join().unwrap();
    }

    println!("done in {:.2}s", start_time.elapsed().as_secs_f32());
}

fn start_worker_thread(
    thread_id: usize,
    send: Sender<(usize, bool)>,
    recv: Receiver<usize>,
    cli: Cli,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let pot = CookingPot::new().unwrap();
        while let Ok(id) = recv.recv() {
            let did_work = dump_chunk(&pot, id, &cli);
            let _ = send.send((thread_id, did_work));
        }
        drop(send);
    })
}

fn dump_chunk(pot: &CookingPot, chunk_id: usize, cli: &Cli) -> bool {
    let data_path = Path::new("./data");
    let data_path = data_path.join(format!("chunk_{}.rawdat", chunk_id));
    if cli.keep && data_path.exists() {
        println!("chunk {} already exists, skipping", chunk_id);
        return false;
    }

    let mut writer = BufWriter::new(File::create(data_path).unwrap());

    let chunk_start = chunk_id * rdata::CHUNK_SIZE;
    let chunk_end = rdata::NUM_TOTAL_RECORDS.min(chunk_start + rdata::CHUNK_SIZE);
    let chunk_size = chunk_end - chunk_start;

    let mut results = Vec::with_capacity(chunk_size);
    for id in chunk_start..chunk_end {
        let data = if id == 0 {
            CookData::invalid()
        } else {
            pot.cook_id(id).unwrap().data
        };
        results.push(data);
    }

    for data in results {
        data.write_to(&mut writer).unwrap();
    }

    true
}


use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::atomic::AtomicUsize;
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Instant;

use botw_recipe_data::{Actor, CookData, RecipeInputs};
use cooking::Cook;

fn main() {
    dump(true);
}

fn dump(skip_existing: bool) {
    let data_path = Path::new("./data");
    if !data_path.exists() {
        std::fs::create_dir_all(data_path).unwrap();
    }

    let total = 5;//botw_recipe_data::CHUNK_COUNT;

    let start_time = Instant::now();
    let num_workers = total.min(num_cpus::get());
    println!("using {} threads", num_workers);

    let (output_send, output_recv) = mpsc::channel();
    let mut input_sends = Vec::with_capacity(num_workers);
    let mut handles = Vec::with_capacity(num_workers);
    for i in 0..num_workers {
        let send = output_send.clone();
        let (input_send, recv) = mpsc::channel();
        let handle = start_worker_thread(i, send, recv, skip_existing);
        // send the first job
        input_send.send(i).unwrap();
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
                let remaining_seconds = if c == 0 {
                    0.0
                } else {
                    let elapsed = start_time.elapsed().as_secs_f32();
                    let speed = c as f32 / elapsed;
                    let remaining = total - c;
                    let recipe_count = c * botw_recipe_data::CHUNK_SIZE;
                    recipe_per_sec = (recipe_count as f32 / elapsed) as usize;
                    remaining as f32 / speed
                };
                println!("chunks finished {}/{} ({:.02} records/s | ETA {:.02}s)", real_count, total, recipe_per_sec, remaining_seconds);
                if real_count >= total {
                    break;
                }
                thread::sleep(std::time::Duration::from_secs(2));
            }
        })
    };
    let mut next = num_workers;
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
    println!("shutting down threads");
    for handle in handles {
        handle.join().unwrap();
    }

    println!("done in {:.2}s", start_time.elapsed().as_secs_f32());
}

fn start_worker_thread(
    thread_id: usize,
    send: Sender<(usize, bool)>,
    recv: Receiver<usize>,
    skip_existing: bool,
) -> JoinHandle<()> {
    thread::spawn(move || {
        println!("thread {} started", thread_id);
        let cook = Cook::new().unwrap();
        while let Ok(id) = recv.recv() {
            let did_work = dump_chunk(&cook, id, skip_existing);
            let _ = send.send((thread_id, did_work));
        }
        drop(send);
        println!("thread {} finished", thread_id);
    })
}

fn dump_chunk(cook: &Cook, chunk_id: usize, skip_existing: bool) -> bool {
    let data_path = Path::new("./data");
    let data_path = data_path.join(format!("chunk_{}.rawdat", chunk_id));
    if skip_existing && data_path.exists() {
        println!("chunk {} already exists, skipping", chunk_id);
        return false;
    }

    let mut writer = BufWriter::new(File::create(data_path).unwrap());

    let chunk_start = chunk_id * botw_recipe_data::CHUNK_SIZE;
    let chunk_end = botw_recipe_data::NUM_TOTAL_RECORDS.min(chunk_start + botw_recipe_data::CHUNK_SIZE);
    let chunk_size = chunk_end - chunk_start;

    let mut results = Vec::with_capacity(chunk_size);
    let mut ingr = Vec::new();
    for id in chunk_start..chunk_end {
        let inputs = RecipeInputs::from_id(id).unwrap();
        ingr.clear();
        for group in inputs.iter() {
            let actor = group.first_actor();
            if actor != Actor::None {
                ingr.push(actor.name());
            }
        }
        let data = if ingr.is_empty() {
            CookData::invalid()
        } else {
            let recipe = cook.cook(&ingr).unwrap();
            rdump_emulate::convert_recipe(&recipe)
        };
        results.push(data);
    }

    for data in results {
        data.write_to(&mut writer).unwrap();
    }

    true
}


use std::io::{self, Write};
use std::mem;
use std::sync::{Arc, atomic::{AtomicBool, AtomicU64, Ordering}};
use std::time;
use std::thread::{self, JoinHandle};

use crate::recipe::{self, RecipeConverter, NUM_ITEMS};
use crate::sys;
use crate::view;

use super::CritMgr;
use super::RecipeData;
use super::Query;
use super::{CHUNK_SIZE, RECORD_SIZE, NUM_RECORD};
const THREADS: usize = 32;

fn multi_process<S, R, H>(running: Arc<AtomicBool>, report_found: bool, total: u64, spawner: S, mut handler: H)
where   S: Fn(usize, Arc<AtomicU64>, Arc<AtomicU64>) -> JoinHandle<R>,
        H: FnMut(R) -> Result<(), String>
{
    println!("--> Processing with {} threads...", THREADS);
    let start_time = time::Instant::now();
    running.store(true, Ordering::SeqCst);

    let processed = Arc::new(AtomicU64::new(0));
    let found = Arc::new(AtomicU64::new(0));
    let mut threads = vec![];
    // spawn threads
    for chunk_id in 0..THREADS {
        let handle = spawner(chunk_id, processed.clone(), found.clone());
        threads.push(handle);
    }
    // spawn a thread to report progress
    let report_handle = if report_found {
        run_reporter(running.clone(), processed, Some(found), total)
    } else {
        run_reporter(running.clone(), processed, None, total)
    };
    // join each thread
    for thread in threads {
        match thread.join() {
            Ok(result) => {
                if let Err(msg) = handler(result) {
                    println!("--> error: {}", msg);
                }
            },
            Err(_) => {
                println!("--> error: thread panicked");
            }
        }
    }
    // join reporter thread
    let is_interrupted = !running.load(Ordering::SeqCst);
    // mark running as false so the reporter can terminate
    running.store(false, Ordering::SeqCst);

    if report_handle.join().is_err() {
        println!("--> error: thread panicked");
    }
    let duration = start_time.elapsed();

    if is_interrupted {
        println!("--> Process interrupted after {:?}.", duration);
    } else {
        println!("--> Process finished in {:?}.", duration);
    }
}

/// Run query on the whole database
pub fn run_query(query: &Query, output: &str, running: Arc<AtomicBool>) -> Result<(), String> {
    sys::create_temp_dir()?;

    multi_process(running.clone(), true, NUM_RECORD, |chunk_id, processed, found| {
        run_query_for_chunk(chunk_id, query.clone(), running.clone(), processed, found)
    }, |result| result);
    
    // combine temporary outputs
    let output = sys::bin_file(output);
    println!("Saving results to {}", &output);
    sys::combine_output(&output, THREADS)?;

    Ok(())    
}

/// Run material sampling on search results
pub fn run_sample(input: &str, converter: &RecipeConverter, running: Arc<AtomicBool>) -> Result<(), String>  {
    sys::create_temp_dir()?;
    let total_recipe_count = sys::split_input(input, THREADS)?;

    let mut output = [0u64; recipe::NUM_ITEMS];

    multi_process(running.clone(), false, total_recipe_count, |chunk_id, processed, _| {
        run_sample_for_chunk(chunk_id, running.clone(), processed.clone())
    }, |result| {
        result.map(|result| {
            for i in 0..NUM_ITEMS {
                output[i] += result[i];
            }
        })
    });

    for i in 0..NUM_ITEMS {
        let count = output[i];
        if count > 0 {
            let percentage = sys::percentage(count, total_recipe_count);
            let material = view::material(i, &converter.get_material_name(i));
            println!("{} in {}/{} recipes ({})", &material, count, total_recipe_count, percentage);
        }
    }

    Ok(())
    
}

pub fn run_reduce(query: &Query, input: &str, output: &str, running: Arc<AtomicBool>) -> Result<(), String> {
    sys::create_temp_dir()?;
    let total_recipe_count = sys::split_input(input, THREADS)?;

    multi_process(running.clone(), true, total_recipe_count, |chunk_id, processed, found| {
        run_reduce_for_chunk(chunk_id, query.clone(), running.clone(), processed.clone(), found.clone())
    }, |result| result);

    // combine temporary outputs
    let output = sys::bin_file(output);
    println!("Saving results to {}", &output);
    sys::combine_output(&output, THREADS)?;

    Ok(())
}

fn run_reporter(running: Arc<AtomicBool>, processed: Arc<AtomicU64>, found: Option<Arc<AtomicU64>>, total: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        let sleep_duration = time::Duration::from_secs(1);
        loop {
            let processed_value = processed.load(Ordering::SeqCst);
            let percentage = sys::percentage(processed_value, total);
            match found {
                Some(ref found) => {
                    let found_value = found.load(Ordering::SeqCst);
                    print!("\r--> Progress: {}/{} ({}). Found {} Recipes   ", processed_value, total, percentage, found_value);
                },
                None => {
                    print!("\r--> Progress: {}/{} ({}).   ", processed_value, total, percentage);
                }
            }
            io::stdout().flush().unwrap();

            if !running.load(Ordering::SeqCst) {
                break;
            }

            thread::sleep(sleep_duration);
        }
        println!("")
    })
}

fn run_query_for_chunk(chunk_id: usize, query: Query, running: Arc<AtomicBool>, processed: Arc<AtomicU64>, found: Arc<AtomicU64>) -> JoinHandle<Result<(), String>> {    
    thread::spawn(move || {
        // open temporary output file
        let output_path = sys::thread_temp_out_file(chunk_id);
        let mut output_writer = sys::open_file_wbuf(&output_path)?;
        // open main db reader
        let mut input_reader = super::database::get_chunk_seq(chunk_id)?;
        // open crit db
        let mut crit_mgr = CritMgr::new();
        // open recipe converter
        let converter = RecipeConverter::new();
        // if we should compare material. skipping comparing material is faster
        let compare_material = !query.exclude_materials.is_empty();

        let mut next_id = chunk_id as u64 * CHUNK_SIZE as u64;

        sys::read_file(&format!("main db chunk {}", chunk_id), &mut input_reader, |bytes_read, buf| {
            let recipe_count = bytes_read / RECORD_SIZE;
            let mut out_buffer = Vec::with_capacity(recipe_count);
            for i in 0..recipe_count {
                // parse the recipe id
                let current_id = next_id;
                next_id+=1;
                // get the recipe data
                let data = RecipeData::from_bytes(&buf[i*RECORD_SIZE..(i+1)*RECORD_SIZE]);
                let crit_hp = if query.crit {
                    crit_mgr.get_recipe_crit_hp(current_id, data.hp)
                } else {
                    data.hp
                };
                // match data against query
                if !query.data_matches(&data, crit_hp) {
                    continue;
                }
                // match data against materials
                if compare_material {
                    let material_set = converter.to_material_set(current_id);
                
                    if !query.materials_matches(material_set) {
                        continue;
                    }
                }
                // if recipe is good, add to output
                out_buffer.push(current_id);
            }

            // write output
            let out_bytes: Vec<u8> = out_buffer.iter().flat_map(|id|id.to_be_bytes()).collect();
            sys::write_file(&output_path, &mut output_writer, &out_bytes)?;

            // report progress
            processed.fetch_add((bytes_read/2) as u64, Ordering::SeqCst);
            found.fetch_add(out_buffer.len() as u64, Ordering::SeqCst);
            
            // maybe interrupt
            if !running.load(Ordering::SeqCst) {
                return Err(format!("thread {} is interrupted", chunk_id));
            }
            Ok(())
        })?;

        sys::write_finish(&output_path, &mut output_writer)?;
        Ok(()) 
    })
}

fn recipe_id_from_bytes(offset: usize, buf: &[u8]) -> u64 {
    let start = offset*mem::size_of::<u64>();
    let mut bytes = [0u8; mem::size_of::<u64>()];
    bytes.clone_from_slice(&buf[start..start+mem::size_of::<u64>()]);
    u64::from_be_bytes(bytes)
}

fn run_sample_for_chunk(chunk_id: usize, running: Arc<AtomicBool>, processed: Arc<AtomicU64>) -> JoinHandle<Result<[u64; recipe::NUM_ITEMS], String>> {
    thread::spawn(move || {
        // open temporary input file
        let input_path = sys::thread_temp_in_file(chunk_id);
        let mut input_reader = sys::open_file_rbuf(&input_path)?;
        // create recipe converter
        let converter = RecipeConverter::new();
        // output buffer
        let mut output = [0u64; recipe::NUM_ITEMS];

        sys::read_file(&input_path, &mut input_reader, |bytes_read, buf| {
            let recipe_count = bytes_read / mem::size_of::<u64>();
            for i in 0..recipe_count {
                // parse the recipe id
                let recipe_id = recipe_id_from_bytes(i, buf);
                
                // parse the materials
                let material_set = converter.to_material_set(recipe_id);
                for material in &material_set {
                    output[material] += 1;
                }
            }
            // report progress
            processed.fetch_add(recipe_count as u64, Ordering::SeqCst);
            // maybe interrupt
            if !running.load(Ordering::SeqCst) {
                return Err(format!("thread {} is interrupted", chunk_id));
            }
            Ok(())
        })?;
       
        Ok(output) 
    })
}

fn run_reduce_for_chunk(chunk_id: usize, query: Query, running: Arc<AtomicBool>, processed: Arc<AtomicU64>, found: Arc<AtomicU64>) -> JoinHandle<Result<(), String>> {
    thread::spawn(move || {
        // open temporary input
        let input_path = sys::thread_temp_in_file(chunk_id);
        let mut input_reader = sys::open_file_rbuf(&input_path)?;
        // open temporary output
        let output_path = sys::thread_temp_out_file(chunk_id);
        let mut output_writer = sys::open_file_wbuf(&output_path)?;
        // create recipe converter
        let converter = RecipeConverter::new();

        sys::read_file(&input_path, &mut input_reader, |bytes_read, buf|{
            let recipe_count = bytes_read / mem::size_of::<u64>();
            let mut out_buffer = Vec::with_capacity(recipe_count);
            
            for i in 0..recipe_count {
                // parse the recipe id
                let recipe_id = recipe_id_from_bytes(i, buf);
                
                // parse the materials
                let material_set = converter.to_material_set(recipe_id);
                if query.materials_matches(material_set) {
                    out_buffer.push(recipe_id);
                }
            }

            // write output
            let out_bytes: Vec<u8> = out_buffer.iter().flat_map(|id|id.to_be_bytes()).collect();
            sys::write_file(&output_path, &mut output_writer, &out_bytes)?;

            // report progress
            processed.fetch_add(recipe_count as u64, Ordering::SeqCst);
            found.fetch_add(out_buffer.len() as u64, Ordering::SeqCst);
            
            // maybe interrupt
            if !running.load(Ordering::SeqCst) {
                return Err(format!("thread {} is interrupted", chunk_id));
            }

            Ok(())
        })?;
 
        sys::write_finish(&output_path, &mut output_writer)?;
        Ok(()) 
    })
}

pub fn run_dump(input: &str, output: &str, converter: &RecipeConverter, crit_mgr: &mut CritMgr) -> Result<(), String>{
    // open binary input
    let input_path = sys::bin_file(input);
    let mut input_reader = sys::open_file_rbuf(&input_path)?;
    // open yaml output
    let output_path = sys::yaml_file(output);
    let mut output_writer = sys::open_file_wbuf(&output_path)?;
    
    sys::read_file(&input_path, &mut input_reader, |bytes_read, buf| {
        let recipe_count = bytes_read / mem::size_of::<u64>();
        
        for i in 0..recipe_count {
            // parse the recipe id
            let recipe_id = recipe_id_from_bytes(i, buf);
            
            // get the data
            let recipe_data = super::database::get_recipe_by_id(recipe_id)?;
            let crit_hp = crit_mgr.get_recipe_crit_hp(recipe_id, recipe_data.hp);
            let materials = converter.to_materials(recipe_id);

            // write output
            let text_content = view::recipe_detail(&materials, recipe_data, crit_hp);
            let content_bytes = text_content.as_bytes();
            sys::write_file(&output_path, &mut output_writer, content_bytes)?;
        }

        Ok(())
    })?;

    sys::write_finish(&output_path, &mut output_writer)?;
    Ok(())
}

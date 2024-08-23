use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Write, BufReader};
#[cfg(not(windows))]
use std::os::unix::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Instant;

use rdata::Recipe;
use rdata::recipe::RecipeInputs;
use rdata::cook::{CookData, CookDataInvalidReason};
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use serde_json::json;
use threadpool::ThreadPool;
use filetime::FileTime;

macro_rules! print_status {
    ($cur:expr, $total:expr, $label:expr) => {{
        let mut stdout = std::io::stdout().lock();
        let _ = write!(
            &mut stdout,
            "\r{}/{} {}",
            $cur,
            $total,
            $label,
        );
        stdout.flush().unwrap()
    }};
    ($cur:expr, $total:expr, $label:expr, $($arg:tt)*) => {{
        let mut stdout = std::io::stdout().lock();
        let _ = write!(&mut stdout, "\r");
        let _ = writeln!(&mut stdout, $($arg)*);
        let _ = write!(&mut stdout,
            "\r{}/{} {}",
            $cur,
            $total,
            $label,
        );
        stdout.flush().unwrap()
    }};
}

fn main() {
    let options = Options::parse();
    if options.inspect {
        let check = Check::new(options, Cache::empty());
        check.inspect().unwrap();
        return;
    }
    if options.mismatch {
        let check = Check::new(options, Cache::empty());
        check.mismatch().unwrap();
        return;
    }

    let start_time = Instant::now();
    let cache = if options.no_cache {
        Cache::empty()
    } else {
        Cache::load().unwrap()
    };
    let mut check = Check::new(options, cache);
    check.check().unwrap();
    println!("saving cache");
    check.cache.save().unwrap();
    let elapsed = start_time.elapsed().as_secs_f32();
    println!("done in {:.2}s", elapsed);
}

struct Cache {
    time: Option<FileTime>,
    good: HashSet<String>,
}

impl Cache {
    pub fn empty() -> Self {
        Self {
            time: None,
            good: HashSet::new(),
        }
    }

    pub fn load() -> Result<Self, Error> {
        let cache_path = Path::new("cache.json");
        let cache_time = if cache_path.exists() {
            cache_path.metadata().map(
                |m| FileTime::from_last_modification_time(&m)).ok()
        } else {
            None
        };
        let good = if cache_path.exists() {
            let v: Vec<String> = serde_json::from_reader(
                BufReader::new(File::open(cache_path)?))?;
            v.into_iter().collect()
        } else {
            HashSet::new()
        };
        Ok(Self {
            time: cache_time,
            good,
        })
    }

    pub fn is_cached_or_remove(&mut self, path: &Path) -> bool {
        let last = match self.time {
            None => return false,
            Some(last) => last
        };
        let s = path.to_string_lossy().to_string();
        if !self.good.contains(&s) {
            return false;
        }
        let is_uptodate = path
            .metadata()
            .map(|m| FileTime::from_last_modification_time(&m))
            .ok()
            .map(|t| t < last)
            .unwrap_or(false);
        if !is_uptodate {
            self.good.remove(&s);
        }
        is_uptodate
    }

    pub fn add(&mut self, path: &Path) {
        let s = path.to_string_lossy().to_string();
        self.good.insert(s);
    }

    pub fn save(self) -> Result<(), Error> {
        let cache_path = Path::new("cache.json");
        let v: Vec<String> = self.good.into_iter().collect();
        let s = serde_json::to_string_pretty(&v)?;
        std::fs::write(&cache_path, s)?;
        Ok(())
    }
}

#[derive(Parser, Debug, Clone, Copy)]
struct Options {
    /// Delete bad chunks when found
    #[clap(long)]
    purge: bool,

    /// Delete check cache and re-check all chunks
    #[clap(long)]
    no_cache: bool,

    /// Inspection mode
    ///
    /// Show the first error in the first invalid chunk
    #[clap(short, long, conflicts_with = "purge")]
    inspect: bool,

    /// Inspect mismatch
    ///
    /// Show the first mismatch in the first chunk that has mismatch
    #[clap(short, long, conflicts_with = "purge", conflicts_with = "inspect")]
    mismatch: bool,
}

#[derive(Parser, Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum DB {
    Emulate,
    Console,
}

fn chunk_path(path: &Path, i: usize) -> PathBuf {
    path.join(format!("chunk_{}.rawdat", i))
}

struct Check {
    pub cache: Cache,
    pub options: Options,
    emulate_path: PathBuf,
    console_path: PathBuf,
}

impl Check {
    fn new(options: Options, cache: Cache) -> Self {
        Self {
            cache,
            options,
            emulate_path: PathBuf::from("../emulate/data"),
            console_path: PathBuf::from("../console/data"),
        }
    }

    fn check(&mut self) -> Result<(), Error> {
        let (e_start, e_end) = self.check_rawdat(self.emulate_path.clone(), "check emulate")?;
        let (c_start, c_end) = self.check_rawdat(self.console_path.clone(), "check console")?;
        println!();
        let end = e_end.min(c_end);
        let start = e_start.min(c_start).min(end);

        if end == 0 {
            println!("no chunks to check. The first chunk is invalid");
            return Ok(())
        }
        if start == end {
            println!("no chunk changed, skipping compare");
            return Ok(())
        }
        let matched = self.compare_db(start, end)?;
        println!();
        let total = rdata::NUM_TOTAL_RECORDS;
        let percentage = matched as f32 / total as f32 * 100.0;
        let percentage_str = format!("{:.2}%", percentage);
        let message = format!("{}/{} ({})", matched, total, percentage_str);
        let badge = json!({
            "schemaVersion": 1,
            "label": "Validated Recipes (Eqiv. Classes)",
            "message": message,
            "color": "blue"
        });
        println!("{}", message);
        let badge_str = serde_json::to_string(&badge)?;
        let badge_path = Path::new("badge.json");
        std::fs::write(&badge_path, badge_str)?;
        println!("saved badge to {}", badge_path.display());
        Ok(())
    }

    fn inspect(&self) -> Result<(), Error> {
        match self.inspect_path(&self.emulate_path)? {
            Some(error) => {
                println!("emulate: {:#?}", error.recipe);
            }
            None => {
                println!("no errors found in emulate");
            }
        }
        match self.inspect_path(&self.console_path)? {
            Some(error) => {
                println!("console: {:#?}", error.recipe);
            }
            None => {
                println!("no errors found in console");
            }
        }
        Ok(())
    }

    fn inspect_path(&self, path: &Path) -> Result<Option<RecipeErrorData>, Error> {
        let errors_path = path.join("errors.json");
        if !errors_path.exists() {
            return Ok(None);
        }
        let reader = BufReader::new(File::open(errors_path)?);
        let errors: Vec<RecipeErrorData> = serde_json::from_reader(reader)?;
        let error = errors.iter().min_by_key(|e| e.chunk);
        Ok(error.cloned())
    }

    fn mismatch(&self) -> Result<(), Error> {
        let errors_path = Path::new("mismatch.json");
        if !errors_path.exists() {
            println!("no mismatch found");
            return Ok(());
        }
        let reader = BufReader::new(File::open(errors_path)?);
        let errors: Vec<RecipeMismatchData> = serde_json::from_reader(reader)?;
        let error = errors.iter().min_by_key(|e| e.chunk);
        match error {
            Some(error) => {
                println!("{:?}", error.recipe_e.inputs);
                println!("emulate: {:#?}", error.recipe_e.data);
                println!("console: {:#?}", error.recipe_c.data);
            }
            None => {
                println!("no mismatch found");
            }
        }
    
        Ok(())
    
    }

    /// Check chunk rawdat in path and return the first chunk that fails the check
    /// All chunks are still checked even if a chunk fails
    fn check_rawdat(&mut self, path: PathBuf, label: &str) -> Result<(usize, usize), Error> {
        print_status!(0, rdata::CHUNK_COUNT, label);
        let mut first_invalid = rdata::CHUNK_COUNT;
        if !path.exists() {
            print_status!(0, rdata::CHUNK_COUNT, label, "path does not exist");
            return Ok((0, 0));
        }
        let pool = ThreadPool::new(num_cpus::get());
        let (send, recv) = mpsc::channel();

        let chunk_count = rdata::CHUNK_COUNT;
        let chunk_size = rdata::CHUNK_SIZE;
        let last_chunk_size = rdata::LAST_CHUNK_SIZE;

        let mut checked = 0;
        let mut first_to_check = rdata::CHUNK_COUNT;
        let mut not_found_count = 0;

        for i in 0..chunk_count {
            let options = self.options;
            let chunk_size = if i == chunk_count - 1 {
                last_chunk_size
            } else {
                chunk_size
            };
            let chunk_file_size: u64 = (chunk_size * 24).try_into().unwrap();
            let chunk_path = chunk_path(&path, i);
            if !chunk_path.exists() {
                checked+=1;
                // print_status!(checked,rdata::CHUNK_COUNT, label, "Chunk {i}: ----- not found");
                first_invalid = first_invalid.min(i);
                not_found_count += 1;
                continue;
            }
            let meta = std::fs::metadata(&chunk_path)?;
            #[cfg(not(windows))]
            let meta_file_size = meta.size();
            #[cfg(windows)]
            let meta_file_size = meta.file_size();
            if meta_file_size != chunk_file_size {
                checked+=1;
                first_invalid = first_invalid.min(i);
                if options.purge {
                    std::fs::remove_file(&chunk_path)?;
                    print_status!(checked,rdata::CHUNK_COUNT, label, "----- deleted chunk {i}");
                } else {
                    print_status!(checked,rdata::CHUNK_COUNT, label, "Chunk {i}: wrong file size. expected: {chunk_file_size}, actual: {}", meta_file_size);
                }
                continue;
            }
            if self.cache.is_cached_or_remove(&chunk_path) {
                checked+=1;
                continue;
            }
            if first_to_check == rdata::CHUNK_COUNT {
                first_to_check = i;
            }
            let send = send.clone();
            pool.execute(move || {
                let result = check_chunk(i, chunk_size, &chunk_path);
                send.send((i, result)).unwrap();
            });
        }

        drop(send);
        let mut errors_to_emit = Vec::new();
        for (i, result) in recv {
            let chunk_path = chunk_path(&path, i);
            checked += 1;
            match result {
                Ok(_) => {
                    self.cache.add(&chunk_path);
                    print_status!(checked,rdata::CHUNK_COUNT, format!("{} ({} not found)", label, not_found_count));
                }
                Err(err) => {
                    first_invalid = first_invalid.min(i);
                    if self.options.purge {
                        std::fs::remove_file(&chunk_path)?;
                        print_status!(checked,rdata::CHUNK_COUNT, label, "----- deleted chunk {i}");
                    } else {
                        print_status!(checked,rdata::CHUNK_COUNT, label, "Chunk {i} failed: {err}");
                        if let Error::InvalidRecord(record, reason, data) = err {
                            let recipe_id = i * rdata::CHUNK_SIZE + record;
                            let recipe_inputs = RecipeInputs::from_id(recipe_id).unwrap();
                            let error_data = RecipeErrorData {
                                chunk: i,
                                record,
                                reason,
                                recipe_id,
                                recipe: Recipe::new(data, recipe_inputs),
                            };
                            errors_to_emit.push(error_data);
                        }
                    }
                }
            }
        }
        pool.join();
        let errors_path = path.join("errors.json");
        if !errors_to_emit.is_empty() {
            let errors = serde_json::to_string_pretty(&errors_to_emit).unwrap();
            std::fs::write(&errors_path, errors)?;
        } else {
            if errors_path.exists() {
                std::fs::remove_file(&errors_path)?;
            }
            println!();
            println!("no errors found");
        }


        Ok((first_to_check, first_invalid))
    }

    fn compare_db(&self, start_at_chunk: usize, stop_at_chunk: usize) -> Result<usize, Error> {
        let label = format!("compare chunks {start_at_chunk} to {} (inclusive)", stop_at_chunk - 1);
        let total = stop_at_chunk - start_at_chunk;
        print_status!(0, total, label);

        let pool = ThreadPool::new(num_cpus::get());
        let (send, recv) = mpsc::channel();

        let chunk_count = rdata::CHUNK_COUNT;
        let chunk_size = rdata::CHUNK_SIZE;
        let last_chunk_size = rdata::LAST_CHUNK_SIZE;

        for i in start_at_chunk..stop_at_chunk {
            let chunk_size = if i == chunk_count - 1 {
                last_chunk_size
            } else {
                chunk_size
            };
            let path_e = chunk_path(&self.emulate_path, i);
            let path_c = chunk_path(&self.console_path, i);

            let send = send.clone();
            pool.execute(move || {
                let result = compare_chunk(chunk_size, &path_e, &path_c);
                send.send((i, result)).unwrap();
            });
        }

        let mut checked = 0;
        let mut matched = start_at_chunk * rdata::CHUNK_SIZE;

        let mut errors_to_emit = Vec::new();

        drop(send);
        for (i, result) in recv {
            checked += 1;
            match result {
                Ok(matched_count) => {
                    matched += matched_count;
                print_status!(checked, total, label);
                }
                Err(err) => {
                    print_status!(checked, total, label, "Chunk {i} failed: {err}");
                    if let Error::Mismatch(record, data_e, data_c, matched_count) = err {
                        matched += matched_count;
                        let recipe_id = i * rdata::CHUNK_SIZE + record;
                        let recipe_inputs = RecipeInputs::from_id(recipe_id).unwrap();
                        let error_data = RecipeMismatchData {
                            chunk: i,
                            record,
                            recipe_id,
                            recipe_e: Recipe::new(data_e, recipe_inputs),
                            recipe_c: Recipe::new(data_c, recipe_inputs),
                        };
                        errors_to_emit.push(error_data);

                    }
                }
            }
        }
        pool.join();
        let errors_path = Path::new("mismatch.json");
        if !errors_to_emit.is_empty() {
            let errors = serde_json::to_string_pretty(&errors_to_emit).unwrap();
            std::fs::write(&errors_path, errors)?;
        } else {
            if errors_path.exists() {
                std::fs::remove_file(&errors_path)?;
            }
            println!();
            println!("everything matched");
        }


        Ok(matched)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RecipeErrorData {
    pub chunk: usize,
    pub record: usize,
    pub reason: CookDataInvalidReason,
    pub recipe_id: usize,
    pub recipe: Recipe
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RecipeMismatchData {
    pub chunk: usize,
    pub record: usize,
    pub recipe_id: usize,
    pub recipe_e: Recipe,
    pub recipe_c: Recipe
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("!! serialization error")]
    Json(#[from] serde_json::Error),
    #[error("!! io error reading chunk")]
    IoError(#[from] io::Error),
    #[error("!! invalid record at {0}: {1:?}")]
    InvalidRecord(usize, CookDataInvalidReason, CookData),
    #[error("!! first mismatch at {0}")]
    Mismatch(usize, CookData, CookData, usize /*matched_count*/)
}

/// Check if every record in a chunk is valid
///
/// Returns the first invalid record if found
fn check_chunk(id: usize, records: usize, path: &Path) -> Result<(), Error> {
    let mut reader = BufReader::new(File::open(path)?);

    for i in 0..records {
        let data = CookData::read_from(&mut reader)?;
        let invalid_reason = if id == 0 && i == 0 {
            data.is_invalid() 
        } else {
            data.is_normal()
        };
        if let Some(reason) = invalid_reason {
            return Err(Error::InvalidRecord(i, reason, data));
        }
    }
    Ok(())
}

/// Compare if 2 chunks are the same
///
/// Returns the number of records that match and the first mismatch if found
fn compare_chunk(records: usize, path_e: &Path, path_c: &Path) -> Result<usize, Error> {
    let mut reader_e = BufReader::new(File::open(path_e)?);
    let mut reader_c = BufReader::new(File::open(path_c)?);

    let mut mismatch = None;
    let mut matched_count = 0;

    for i in 0..records {
        let data_e = CookData::read_from(&mut reader_e)?;
        let data_c = CookData::read_from(&mut reader_c)?;

        if data_e != data_c {
            if mismatch.is_none() {
                mismatch = Some((i, data_e, data_c));
            }
        } else {
            matched_count += 1;
        }
    }
    if let Some((i, data_e, data_c)) = mismatch {
        return Err(Error::Mismatch(i, data_e, data_c, matched_count));
    }

    Ok(matched_count)
}

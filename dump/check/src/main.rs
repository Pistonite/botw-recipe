use std::fs::File;
use std::io::{self, Write, BufReader};
use std::os::unix::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::mpsc;

use rdata::{Recipe, RecipeInputs};
use rdata::cook::{CookData, CookDataInvalidReason};
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use serde_json::json;
use threadpool::ThreadPool;

macro_rules! print_status {
    ($cur:expr, $label:expr) => {{
        let mut stdout = std::io::stdout().lock();
        let _ = write!(
            &mut stdout,
            "\r{}/{} {}                     ",
            $cur,
            rdata::CHUNK_COUNT,
            $label,
        );
        stdout.flush().unwrap()
    }};
    ($cur:expr, $label:expr, $($arg:tt)*) => {{
        let mut stdout = std::io::stdout().lock();
        let _ = write!(&mut stdout, "\r");
        let _ = writeln!(&mut stdout, $($arg)*);
        let _ = write!(&mut stdout,
            "\r{}/{} {}                      ",
            $cur,
            rdata::CHUNK_COUNT,
            $label,
        );
        stdout.flush().unwrap()
    }};
}

fn main() {
    let options = Options::parse();
    let check = Check::new(options);
    if let Some(chunk_id) = check.options.inspect {
        check.inspect(chunk_id).unwrap();
        return;
    }
    if let Some(chunk_id) = check.options.mismatch {
        check.mismatch(chunk_id).unwrap();
        return;
    }
    check.check().unwrap();
}

#[derive(Parser, Debug, Clone, Copy)]
struct Options {
    /// Delete bad chunks when found
    #[clap(long)]
    purge: bool,

    /// Print verbose output
    #[clap(short, long)]
    verbose: bool,

    /// Inspection mode - looking at errors for a chunk
    #[clap(short, long, conflicts_with = "purge")]
    inspect: Option<usize>,

    /// Specify which database to work on in non-default modes
    #[clap(long, default_value = "emulate")]
    db: DB,

    /// Skip checking emulated dump
    #[clap(long)]
    skip_emulate: bool,

    /// Skip checking console dump
    #[clap(long, conflicts_with = "skip_emulate")]
    skip_console: bool,

    /// Inspect mismatch
    #[clap(short, long)]
    mismatch: Option<usize>
}

#[derive(Parser, Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum DB {
    Emulate,
    Console,
}

struct Check {
    options: Options,
    emulate_path: PathBuf,
    console_path: PathBuf,
}

impl Check {
    fn new(options: Options) -> Self {
        Self {
            options,
            emulate_path: PathBuf::from("../emulate/data"),
            console_path: PathBuf::from("../console/data"),
        }
    }

    fn check(&self) -> Result<(), Error> {
        let e_valid = if self.options.skip_emulate {
            rdata::NUM_TOTAL_RECORDS
        } else {
            self.check_rawdat(&self.emulate_path, "check emulate")?
        };
        let c_valid = if self.options.skip_console {
            rdata::NUM_TOTAL_RECORDS
        } else {
            self.check_rawdat(&self.console_path, "check console")?
        };
        println!();
        println!();
        let valid = e_valid.min(c_valid);

        if valid == 0 {
            println!("no chunks to check. The first chunk is invalid");
            return Ok(());
        }
        let matched = self.compare_db(e_valid.min(c_valid))?;
        println!();
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

    fn inspect(&self, chunk_id: usize) -> Result<(), Error> {
        let path = if self.options.db == DB::Emulate {
            &self.emulate_path
        } else {
            &self.console_path
        };
        let errors_path = path.join("errors.json");
        if !errors_path.exists() {
            println!("no errors found");
            return Ok(());
        }
        let reader = BufReader::new(File::open(errors_path)?);
        let errors: Vec<RecipeErrorData> = serde_json::from_reader(reader)?;
        let error = errors.iter().find(|e| e.chunk == chunk_id);
        match error {
            Some(error) => {
                println!("{:#?}", error);
            }
            None => {
                println!("no errors found in chunk {}", chunk_id);
            }
        }

        Ok(())
    }

    fn mismatch(&self, chunk_id: usize) -> Result<(), Error> {
        let errors_path = Path::new("mismatch.json");
        if !errors_path.exists() {
            println!("no mismatch found");
            return Ok(());
        }
        let reader = BufReader::new(File::open(errors_path)?);
        let errors: Vec<RecipeMismatchData> = serde_json::from_reader(reader)?;
        let error = errors.iter().find(|e| e.chunk == chunk_id);
        match error {
            Some(error) => {
                println!("{:?}", error.recipe_e.inputs);
                println!("emulate: {:#?}", error.recipe_e.data);
                println!("console: {:#?}", error.recipe_c.data);
            }
            None => {
                println!("no errors found in chunk {}", chunk_id);
            }
        }

        Ok(())

    }
    /// Check chunk rawdat in path and return the first chunk that fails the check
    /// All chunks are still checked even if a chunk fails
    fn check_rawdat(&self, path: &Path, label: &str) -> Result<usize, Error> {
        print_status!(0, label);
        let mut first_invalid = rdata::CHUNK_COUNT;
        if !path.exists() {
            print_status!(0, label, "path does not exist");
            return Ok(0);
        }
        let pool = ThreadPool::new(num_cpus::get());
        let (send, recv) = mpsc::channel();

        let chunk_count = rdata::CHUNK_COUNT;
        let chunk_size = rdata::CHUNK_SIZE;
        let last_chunk_size = rdata::LAST_CHUNK_SIZE;

        let mut checked = 0;

        for i in 0..chunk_count {
            let options = self.options;
            let chunk_size = if i == chunk_count - 1 {
                last_chunk_size
            } else {
                chunk_size
            };
            let chunk_file_size: u64 = (chunk_size * 24).try_into().unwrap();
            let chunk_path = path.join(format!("chunk_{}.rawdat", i));
            if !chunk_path.exists() {
                checked+=1;
                print_status!(checked, label, "Chunk {i}: ----- not found");
                first_invalid = first_invalid.min(i);
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
                    print_status!(checked, label, "----- deleted chunk {i}");
                } else {
                    print_status!(checked, label, "Chunk {i}: wrong file size. expected: {chunk_file_size}, actual: {}", meta_file_size);
                }
                continue;
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
            checked += 1;
            match result {
                Ok(_) => {
                    if self.options.verbose {
                        print_status!(checked, label, "Chunk {i}: ok");
                    } else {
                        print_status!(checked, label);
                    }
                }
                Err(err) => {
                    first_invalid = first_invalid.min(i);
                    if self.options.purge {
                        let chunk_path = path.join(format!("chunk_{}.rawdat", i));
                        std::fs::remove_file(&chunk_path)?;
                        print_status!(checked, label, "----- deleted chunk {i}");
                    } else {
                        print_status!(checked, label, "Chunk {i} failed: {err}");
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
        }


        Ok(first_invalid)
    }

    fn compare_db(&self, stop_at_chunk: usize) -> Result<usize, Error> {
        let label = format!("compare chunks ({})", stop_at_chunk);
        print_status!(0, label);

        let pool = ThreadPool::new(num_cpus::get());
        let (send, recv) = mpsc::channel();

        let chunk_count = rdata::CHUNK_COUNT;
        let chunk_size = rdata::CHUNK_SIZE;
        let last_chunk_size = rdata::LAST_CHUNK_SIZE;

        for i in 0..stop_at_chunk {
            let chunk_size = if i == chunk_count - 1 {
                last_chunk_size
            } else {
                chunk_size
            };
            let path_e = self.emulate_path.join(format!("chunk_{}.rawdat", i));
            let path_c = self.console_path.join(format!("chunk_{}.rawdat", i));

            let send = send.clone();
            pool.execute(move || {
                let result = compare_chunk(chunk_size, &path_e, &path_c);
                send.send((i, result)).unwrap();
            });
        }

        let mut checked = 0;
        let mut matched = 0;

        let mut errors_to_emit = Vec::new();

        drop(send);
        for (i, result) in recv {
            checked += 1;
            match result {
                Ok(matched_count) => {
                    matched += matched_count;
                    if self.options.verbose {
                        print_status!(checked, label, "Chunk {i}: ----- matched -----");
                    } else {
                        print_status!(checked, label);
                    }
                }
                Err(err) => {
                    print_status!(checked, label, "Chunk {i} failed: {err}");
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
            println!();
            println!("everything matched");
        }


        Ok(matched)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RecipeErrorData {
    pub chunk: usize,
    pub record: usize,
    pub reason: CookDataInvalidReason,
    pub recipe_id: usize,
    pub recipe: Recipe
}

#[derive(Debug, Serialize, Deserialize)]
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

use std::fs::{self, File};
use std::io::{Read, BufReader, Write, BufWriter};
use std::mem;
use positioned_io::{RandomAccessFile, ReadAt};

const FILE_BUF_SIZE: usize = 4096;

pub fn percentage(current: u64, total: u64) -> String {
    let percentage100 = (current * 10000 / total) as u32;
    let percentage = f64::from(percentage100) / 100.0;
    format!("{}%", percentage)
}

pub fn to_usize(input: u64) -> usize {
    input.try_into().unwrap_or_else(|_| panic!("Cannot convert {} to usize", input))
}

/// Create temp directory
pub fn create_temp_dir() -> Result<(), String> {
    fs::create_dir_all("temp").map_err(|err| format!("Cannot create temp directory: {}", err.to_string()))
}

pub fn remove_temp_dir() {
    if let Err(_) = fs::remove_dir_all("temp") {
        // silently ignore
    }
}

/// File name for temp input to a thread
pub fn thread_temp_in_file(thread_id: usize) -> String {
    format!("temp/thread{}_in.bin", thread_id)
}

/// File name for temp output to a thread
pub fn thread_temp_out_file(thread_id: usize) -> String {
    format!("temp/thread{}_out.bin", thread_id)
}

/// File name for binary input/output
pub fn bin_file(path: &str) -> String {
    add_ext_if_need(path, ".bin")
}

/// File name for yaml input
pub fn yaml_file(path: &str) -> String {
    add_ext_if_need(path, ".yaml")
}

fn add_ext_if_need(path: &str, ext: &str) -> String {
    let mut out_file = String::from(path);
    if !path.ends_with(ext) {
        out_file.push_str(ext);
    }
    out_file
}

pub fn open_random_access_file(path: &str) -> Result<RandomAccessFile, String>{
    RandomAccessFile::open(&path).map_err(|err| format!("Cannot open {} for random access: {}", &path, err.to_string()))
}

pub fn read_random_access_file(path: &str, file: &RandomAccessFile, offset: u64, buf: &mut [u8]) -> Result<usize, String> {
    file.read_at(offset, buf).map_err(|err| format!("error accessing {}: {}", &path, err.to_string()))
}

pub fn open_file_rbuf(path: &str) -> Result<BufReader<File>, String> {
    Ok(BufReader::new(open_file_r(path)?))
}

fn open_file_r(path: &str) -> Result<File, String> {
    File::open(&path).map_err(|err| format!("Cannot open {} for reading: {}", &path, err.to_string()))
}

pub fn open_file_wbuf(path: &str) -> Result<BufWriter<File>, String> {
    let output_file = File::create(&path).map_err(|err| format!("Cannot open file for writing: {}", err.to_string()))?;
    Ok(BufWriter::new(output_file))
}

pub fn read_file<F>(path: &str, reader: &mut dyn Read, mut op: F) -> Result<(), String> 
where F: FnMut(usize, &[u8]) -> Result<(), String>
{
    let mut buffer = [0u8; FILE_BUF_SIZE];
    loop {
        let bytes_read = reader.read(&mut buffer).map_err(|err| format!("error reading {}: {}", path, err.to_string()))?;
        op(bytes_read, &buffer)?;
        if bytes_read != FILE_BUF_SIZE {
            break;
        }
    }

    Ok(())
}

pub fn write_file(path: &str, writer: &mut dyn Write, buf: &[u8]) -> Result<usize, String> {
    writer.write(buf).map_err(|err| format!("error writing {}: {}", &path, err.to_string()))
}

pub fn write_finish(path: &str, writer: &mut dyn Write) -> Result<(), String> {
    writer.flush().map_err(|err| format!("error writing {}: {}", &path, err.to_string()))
}

/// Combine thread temporary output to output file
pub fn combine_output(output: &str, thread_count: usize) -> Result<(), String>{
    // open output file
    let output_path = bin_file(output);
    let mut output_writer = open_file_wbuf(&output_path)?;

    // read each temporary output
    for thread_id in 0..thread_count {
        let thread_temp = thread_temp_out_file(thread_id);
        let mut input_reader = open_file_rbuf(&thread_temp)?;

        // write entire content to output
        read_file(&thread_temp, &mut input_reader, |bytes_read: usize, buf: &[u8]| {
            write_file(&output_path, &mut output_writer, &buf[0..bytes_read]).map(|_|())
        })?;
    }

    write_finish(&output_path, &mut output_writer)?;
    Ok(())
}

/// Split binary input file to thread temporary input
/// Returns how many recipes are in the input file in total
pub fn split_input(input: &str, thread_count: usize) -> Result<u64, String> {
    // open input file
    let input_path = bin_file(input);
    let input_file = open_file_r(&input_path)?;
    let size = input_file.metadata().map_err(|err| format!("Cannot read size of {}: {}", &input_path, err.to_string()))?.len();
    let mut input_reader = BufReader::new(input_file);

    // get total count and count for each thread
    let recipe_count = size / mem::size_of::<u64>() as u64;
    let chunk_size = (recipe_count / thread_count as u64 + 1) * mem::size_of::<u64>() as u64;

    let mut current_thread_id = 0;
    let mut current_size: u64 = 0;
    let mut output_path = thread_temp_in_file(current_thread_id);
    let mut output_writer = open_file_wbuf(&output_path)?;

    read_file(&input_path, &mut input_reader, |bytes_read: usize, buf: &[u8]| {
        write_file(&output_path, &mut output_writer, &buf[0..bytes_read])?;
        current_size += bytes_read as u64;
        // switch to a different file
        if current_size >= chunk_size {
            write_finish(&output_path, &mut output_writer)?;
            current_size = 0;
            current_thread_id+=1;
            if current_thread_id >= thread_count {
                panic!("Current thread id {} exceed max thread count {}", current_thread_id, thread_count);
            }
            output_path = thread_temp_in_file(current_thread_id);
            output_writer = open_file_wbuf(&output_path)?;
        }
        Ok(())
    })?;
    
    write_finish(&output_path, &mut output_writer)?;
    current_thread_id+=1;
    while current_thread_id < thread_count {
        output_path = thread_temp_in_file(current_thread_id);
        open_file_wbuf(&output_path)?;
        current_thread_id+=1;
    }

    Ok(recipe_count)
}

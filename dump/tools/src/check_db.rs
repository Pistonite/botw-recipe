use std::{fs::File, io::{Read, Seek, SeekFrom}, path::{Path, PathBuf}};

use clap::Parser;
use rdata::{cook::CookData, db::{CritDb, Record}, recipe::RecipeInputs};
use rcook::CookingPot;

fn main() {
    let options = Options::parse();
    check_db(options.chunk, options.id);
}

#[derive(Parser)]
struct Options {
    /// The compact chunk id
    chunk: usize,
    /// The record id in that chunk
    id: usize,
}

fn check_db(chunk: usize, id: usize) {
    let recipe_id = chunk * rdata::COMPACT_CHUNK_SIZE + id;
    let raw_chunk = recipe_id / rdata::CHUNK_SIZE;
    let raw_chunk_recipe_id = recipe_id % rdata::CHUNK_SIZE;
    let recipe_inputs = RecipeInputs::from_id(recipe_id).unwrap();
    println!("Inputs: {:#?}", recipe_inputs);
    let pot = CookingPot::new().unwrap();
    let cook_data = pot.cook_id(recipe_id).unwrap();
    println!("Cooked: {:#?}", cook_data);

    let emulate_path = PathBuf::from(format!("../emulate/data/chunk_{}.rawdat", raw_chunk));
    if emulate_path.exists() {
        let offset = raw_chunk_recipe_id * 24;
        let mut file = File::open(emulate_path).unwrap();
        file.seek(SeekFrom::Start(offset as u64)).unwrap();
        let data = CookData::read_from(&mut file).unwrap();
        println!("Emulated DB: {:#?}", data);
    } else {
        println!("emulated data not found");
    }

    let console_path = PathBuf::from(format!("../console/data/chunk_{}.rawdat", raw_chunk));
    if console_path.exists() {
        let offset = raw_chunk_recipe_id * 24;
        let mut file = File::open(console_path).unwrap();
        file.seek(SeekFrom::Start(offset as u64)).unwrap();
        let data = CookData::read_from(&mut file).unwrap();
        println!("Console DB: {:#?}", data);
    } else {
        println!("console data not found");
    }

    let compact_path = PathBuf::from(format!("../emulate/compact/chunk_{}.rdb", chunk));
    if compact_path.exists() {
        let offset = id * 2;
        let mut file = File::open(compact_path).unwrap();
        file.seek(SeekFrom::Start(offset as u64)).unwrap();
        let mut buf = [0; 2];
        file.read_exact(&mut buf).unwrap();
        let data = Record::from(u16::from_le_bytes(buf));
        let hp = data.value();
        let price = data.modifier();
        let crit_db = CritDb::open("../emulate/compact/crit.db").unwrap();
        println!("Compact DB: hp = {}, price = {}, crit_rng_hp = {}", hp, price, crit_db.get(recipe_id));
    } else {
        println!("compact data not found");
    }
}

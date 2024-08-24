use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{db::ChunkReader, recipe::{RecipeId, RecipeInputs}, wmc::WeaponModifier, Group};

use super::{CritDb, Error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    /// The chunk id
    pub chunk: usize,
    /// The maximum HP of all records in the chunk, without crit_rng_hp
    pub max_hp: i32,
    /// The maximum HP of all records in the chunk, with crit_rng_hp
    pub max_hp_crit_rng: i32,
    /// Mask for if any record in the chunk has the modifier
    pub includes_modifier: u32,
    /// Mask for if all records in the chunk have the modifier
    pub all_includes_modifier: u32,
    pub min_price: u32,
    pub max_price: u32,
    pub min_hp: i32,
    // pub all_pe_only: bool,
    /// Integrity property of the chunk
    #[serde(flatten)]
    pub integrity: Integrity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Integrity property of the chunk
pub enum Integrity {
    /// SHA256 hash of the chunk
    SHA256(String),
    /// If all records in the chunk have the same value
    AllSame(i32, i32, bool),
}

impl Index {
    pub fn compute<P: AsRef<Path>>(crit_db: &CritDb, chunk_id: usize, path: P) -> Result<Self, Error> {
        let chunk = ChunkReader::open(chunk_id, path)?;
        let mut min_hp = i32::MAX;
        let mut max_hp = i32::MIN;
        let mut max_hp_crit_rng = i32::MIN;
        let mut min_price = u32::MAX;
        let mut max_price = 0u32;
        let mut includes_modifier = 0u32;
        let mut all_includes_modifier = u32::MAX;

        let mut is_all_same = true;
        let mut same_record = None;
        let mut same_crit_rng_hp = true;

        // let mut all_pe_only = true;

        let recipe_start = chunk_id * crate::COMPACT_CHUNK_SIZE;
        let total = if chunk_id == crate::COMPACT_CHUNK_COUNT - 1 {
            crate::COMPACT_LAST_CHUNK_SIZE
        } else {
            crate::COMPACT_CHUNK_SIZE
        };
        let mut hasher = Sha256::new();
        let recipe_end = recipe_start + total;
        for (recipe_id, record) in (recipe_start..recipe_end).zip(chunk) {
            let record = record?;
            hasher.update(&record.raw().to_le_bytes());
            let crit_rng_hp = crit_db.get(recipe_id);
            if is_all_same {
                if let Some(same_record) = same_record {
                    if record != same_record || crit_rng_hp != same_crit_rng_hp {
                        is_all_same = false;
                    }
                } else {
                    same_record = Some(record);
                    same_crit_rng_hp = crit_rng_hp;
                }
            }
            // if all_pe_only {
            //     let inputs: RecipeInputs = RecipeId::new(recipe_id).unwrap().into();
            //     let mut this_needs_pe = false;
            //     for group in inputs.as_slice() {
            //         if group.contains_pe_only() {
            //             this_needs_pe = true;
            //             break;
            //         }
            //     }
            //     if !this_needs_pe {
            //         all_pe_only = false;
            //     }
            // }
            let hp = record.value();
            max_hp = max_hp.max(hp);
            min_hp = min_hp.min(hp);
            if crit_rng_hp {
                let crit_hp = (hp + 12).min(120);
                max_hp_crit_rng = max_hp_crit_rng.max(crit_hp);
            }
            let modifier = record.modifier();
            min_price = min_price.min(modifier);
            max_price = max_price.max(modifier);
            includes_modifier = includes_modifier | modifier;
            all_includes_modifier = all_includes_modifier & modifier;
        }

        let hash = hasher.finalize();

        max_hp_crit_rng = max_hp_crit_rng.max(max_hp);
        let integrity = if is_all_same {
            if let Some(record) = same_record {
                let value = record.value();
                let modifier = record.modifier();
                let crit_rng_hp = same_crit_rng_hp;
                Integrity::AllSame(value, modifier as i32, crit_rng_hp)
            } else {
                Integrity::SHA256(base16ct::lower::encode_string(&hash))
            }
        } else {
            Integrity::SHA256(base16ct::lower::encode_string(&hash))
        };

        Ok(Self {
            min_hp,
            min_price,
            max_price,
            chunk: chunk_id,
            max_hp,
            max_hp_crit_rng,
            includes_modifier,
            all_includes_modifier,
            // all_pe_only,
            integrity,
        })
    }

    pub fn is_all_same(&self) -> bool {
        match &self.integrity {
            Integrity::AllSame(_, _, _) => true,
            _ => false,
        }
    }
}

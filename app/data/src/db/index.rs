use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::cook::{CookData, CookEffect};
use crate::wmc::WeaponModifierSet;

use super::{Filter, Record};

/// Index metadata for a chunk. Used to skip chunks when searching for recipes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    /// The chunk id
    pub chunk: usize,
    /// The minimum value (HP) of all records in the chunk
    pub min_value: i32,
    /// The maximum value (HP) of all records in the chunk, without considering crit_rng_hp
    pub max_value: i32,
    /// The maximum value (HP) of all records in the chunk, with crit_rng_hp
    pub max_value_crit_rng: i32,
    /// Mask for if any record in the chunk has the modifier
    pub includes_modifier: WeaponModifierSet,
    /// Mask for if all records in the chunk have the modifier
    pub all_includes_modifier: WeaponModifierSet,
    /// Minimum price of all records in the chunk
    ///
    /// This is not used against matching, just for debugging
    pub min_price: u32,
    /// Maximum price of all records in the chunk
    ///
    /// This is not used against matching, just for debugging
    pub max_price: u32,
    /// SHA256 hash of the chunk
    pub sha256: String,
}

pub struct IndexBuilder {
    chunk: usize,
    hasher: Sha256,
    min_value: i32,
    max_value: i32,
    max_value_crit_rng: i32,
    includes_modifier: WeaponModifierSet,
    all_includes_modifier: WeaponModifierSet,
    min_price: u32,
    max_price: u32,
}

impl IndexBuilder {
    pub fn new(chunk_id: usize) -> Self {
        Self {
            chunk: chunk_id,
            hasher: Sha256::new(),
            min_value: i32::MAX,
            max_value: i32::MIN,
            max_value_crit_rng: i32::MIN,
            includes_modifier: WeaponModifierSet::new(),
            all_includes_modifier: WeaponModifierSet::all(),
            min_price: u32::MAX,
            max_price: 0,
        }
    }

    pub fn update(&mut self, data: &CookData, crit_rng_hp: bool) {
        let record = Record::from_data(data, crit_rng_hp);
        self.hasher.update(u16::from(record).to_le_bytes());

        let value = record.value();
        self.max_value = self.max_value.max(value);
        self.min_value = self.min_value.min(value);
        if crit_rng_hp {
            let add = if data.effect_id == CookEffect::LifeMaxUp.game_repr_f32() {
                4
            } else {
                12
            };
            let crit_value = (value + add).min(120);
            self.max_value_crit_rng = self.max_value_crit_rng.max(crit_value);
        }
        let modifier = record.modifier();
        self.min_price = self.min_price.min(modifier);
        self.max_price = self.max_price.max(modifier);
        self.includes_modifier = self.includes_modifier.union(modifier);
        self.all_includes_modifier = self.all_includes_modifier.intersection(modifier);
    }

    pub fn build(self) -> Index {
        // max crit value is at least max value
        let max_value_crit_rng = self.max_value_crit_rng.max(self.max_value);

        let hash = self.hasher.finalize();
        let hash = base16ct::lower::encode_string(&hash);

        Index {
            chunk: self.chunk,
            min_value: self.min_value,
            max_value: self.max_value,
            max_value_crit_rng,
            includes_modifier: self.includes_modifier,
            all_includes_modifier: self.all_includes_modifier,
            min_price: self.min_price,
            max_price: self.max_price,
            sha256: hash,
        }
    }
}

impl Index {
    /// Return true if none of the records in this chunk match the filter
    ///
    /// Note that returning false can still mean that the chunk has no matching records
    pub fn can_skip(&self, filter: &Filter) -> bool {
        let self_max = if filter.include_crit_rng_hp {
            self.max_value_crit_rng
        } else {
            self.max_value
        };

        if self_max < filter.min_value {
            return true;
        }
        if self.min_value > filter.max_value {
            return true;
        }
        if self
            .includes_modifier
            .intersection(filter.includes_modifier)
            != filter.includes_modifier
        {
            // none of recipes have all the required modifiers
            return true;
        }
        if self
            .all_includes_modifier
            .intersection(filter.excludes_modifier)
            != WeaponModifierSet::new()
        {
            // all recipes have at least one of the excluded modifiers
            return true;
        }

        false
    }
}

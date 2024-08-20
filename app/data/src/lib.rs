mod generated;

use std::io;
use std::io::Write;

pub use generated::{NUM_GROUPS, NUM_INGR, NUM_TOTAL_RECORDS, CHUNK_SIZE, CHUNK_COUNT, LAST_CHUNK_SIZE};
pub use generated::{Actor, Group};

mod recipe;
pub use recipe::*;
use serde::{Deserialize, Serialize};

/// Get the number of ways to choose `k` items from `n` items, allowing for repetition
///
/// The time complexity is O(1) because all values are pre-computed.
pub fn multichoose(n: usize, k: usize) -> usize {
    generated::MULTICHOOSE[n][k]
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    pub data: CookData,
    pub inputs: Vec<String>,
}

impl Recipe {
    pub fn new(data: CookData, inputs: RecipeInputs) -> Self {
        Self {
            data,
            inputs: inputs.to_names().into_iter().map(|a| a.to_string()).collect(),
        }
    }
}

// use std::fs;
// use bit_set::BitSet;
/// This data mirrors uking::ui::PouchItem::CookData, with an extra crit_chance field
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct CookData {
    /// Number of quarter-hearts. Usually 0-120
    pub health_recover: i32,
    /// Effect duration in seconds, usually 0-1800
    pub effect_duration: i32,
    /// Price
    pub sell_price: i32,
    /// Effect ID, but a float for some reason. -1 is None
    pub effect_id: f32,
    /// Effect level, usually 0-3, higher for hearty
    pub effect_level: f32,
    /// crit chance, usually 0-100
    pub crit_chance: i32
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "reason", content = "data")]
pub enum CookDataInvalidReason {
    ExpectedInvalid,
    InvalidHealthRecover(i32),
    InvalidEffectDuration(i32),
    NegativeSellPrice(i32),
    NegativeEffectLevel(f32),
    NonIntegerEffectLevel(f32),
    InvalidNoneEffect {
        sell_price: i32,
        effect_level: f32
    },
    LifeRecover,
    LifeMaxUpTooHigh(f32),
    EffectLevelTooHigh(f32),
    UnknownEffect(f32),
}

impl CookData {
    pub fn write_to<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&self.health_recover.to_le_bytes())?;
        w.write_all(&self.effect_duration.to_le_bytes())?;
        w.write_all(&self.sell_price.to_le_bytes())?;
        w.write_all(&self.effect_id.to_le_bytes())?;
        w.write_all(&self.effect_level.to_le_bytes())?;
        w.write_all(&self.crit_chance.to_le_bytes())?;
        Ok(())
    }

    pub fn read_from<R: io::Read>(r: &mut R) -> io::Result<Self> {
        let mut data = [0u8; 4];
        r.read_exact(&mut data)?;
        let health_recover = i32::from_le_bytes(data);
        r.read_exact(&mut data)?;
        let effect_duration = i32::from_le_bytes(data);
        r.read_exact(&mut data)?;
        let sell_price = i32::from_le_bytes(data);
        r.read_exact(&mut data)?;
        let effect_id = f32::from_le_bytes(data);
        r.read_exact(&mut data)?;
        let effect_level = f32::from_le_bytes(data);
        r.read_exact(&mut data)?;
        let crit_chance = i32::from_le_bytes(data);
        Ok(Self {
            health_recover,
            effect_duration,
            sell_price,
            effect_id,
            effect_level,
            crit_chance
        })
    }
    /// Return an invalid CookData with all 0 bytes
    pub fn invalid() -> Self {
        Self {
            health_recover: 0,
            effect_duration: 0,
            sell_price: 0,
            effect_id: 0.0,
            effect_level: 0.0,
            crit_chance: 0
        }
    }

    pub fn is_invalid(&self) -> Option<CookDataInvalidReason> {
        if self == &Self::invalid() {
            None
        } else {
            Some(CookDataInvalidReason::ExpectedInvalid)
        }
    }

    /// Get that the data are in their normal ranges
    pub fn is_normal(&self) -> Option<CookDataInvalidReason> {
        if self.health_recover < 0 || self.health_recover > 120 {
            return Some(CookDataInvalidReason::InvalidHealthRecover(self.health_recover));
        }
        if self.effect_duration < 0 || self.effect_duration > 1800 {
            return Some(CookDataInvalidReason::InvalidEffectDuration(self.effect_duration));
        }
        if self.sell_price < 0 {
            return Some(CookDataInvalidReason::NegativeSellPrice(self.sell_price));
        }
        if self.effect_level < 0.0 {
            return Some(CookDataInvalidReason::NegativeEffectLevel(self.effect_level));
        }
        if self.effect_level.round() != self.effect_level {
            return Some(CookDataInvalidReason::NonIntegerEffectLevel(self.effect_level));
        }
        match self.effect_id {
            MOD_NONE => {
                if self.effect_level != 1.0 {
                    // must be fairy tunic
                    if self.sell_price != 2 {
                        return Some(CookDataInvalidReason::InvalidNoneEffect {
                            sell_price: self.sell_price,
                            effect_level: self.effect_level
                        });
                    }
                }
            }
            MOD_LIFE_RECOVER => {
                //supposed to be unused
                return Some(CookDataInvalidReason::LifeRecover);
            }
            MOD_LIFE_MAX_UP => {
                if self.effect_level > 25.0 {
                    // max is 5 big hearty radish which gives 25 hearts
                    return Some(CookDataInvalidReason::LifeMaxUpTooHigh(self.effect_level));
                }
            }
            MOD_RESIST_HOT | MOD_RESIST_COLD | MOD_FIREPROOF=> {
                if self.effect_level> 2.0 {
                    return Some(CookDataInvalidReason::EffectLevelTooHigh(self.effect_level));
                }
            }
            MOD_RESIST_ELECTRIC |
MOD_ATTACK_UP | MOD_DEFENSE_UP | MOD_QUIETNESS | MOD_MOVING_SPEED
=> {
                if self.effect_level > 3.0 {
                    return Some(CookDataInvalidReason::EffectLevelTooHigh(self.effect_level));
                }
            }
            MOD_GUTS_RECOVER => {
                if self.effect_level > 1.0 {
                    return Some(CookDataInvalidReason::EffectLevelTooHigh(self.effect_level));
                }
            }
            MOD_EX_GUTS_MAX_UP => {
                if self.effect_level > 1.0 {
                    return Some(CookDataInvalidReason::EffectLevelTooHigh(self.effect_level));
                }
            }
            _ => {
                return Some(CookDataInvalidReason::UnknownEffect(self.effect_id));
            }
        }

        None
    }
}
const MOD_LIFE_RECOVER :f32= 1.0;
const MOD_LIFE_MAX_UP :f32= 2.0;
const MOD_RESIST_HOT :f32= 4.0;
const MOD_RESIST_COLD :f32= 5.0;
const MOD_RESIST_ELECTRIC :f32= 6.0;
const MOD_ATTACK_UP: f32 = 10.0;
const MOD_DEFENSE_UP :f32= 11.0;
const MOD_QUIETNESS :f32= 12.0;
const MOD_MOVING_SPEED :f32= 13.0;
const MOD_GUTS_RECOVER :f32= 14.0;
const MOD_EX_GUTS_MAX_UP :f32= 15.0;
const MOD_FIREPROOF :f32= 16.0;
const MOD_NONE :f32= -1.0;

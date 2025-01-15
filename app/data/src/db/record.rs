use std::io::Write;

use botw_recipe_sys::{num_ingr, Group, GroupMnr, CookEffect};

use crate::cook::{CookData, CookingPot};
use crate::wmc::WeaponModifierSet;

use super::{Error, Filter};

/// Holder of the record with its recipe id
pub struct PositionedRecord {
    /// The recipe id
    pub recipe_id: u64,
    /// The record
    pub record: Record,
}

impl PositionedRecord {
    pub fn matches(&self, filter: &Filter, pot: &CookingPot) -> Result<bool, Error> {
        let value = self.record.value();
        if value > filter.max_value {
            return Ok(false);
        }
        let modifier = self.record.modifier_set();
        if filter.includes_modifier.intersection(modifier) != filter.includes_modifier {
            return Ok(false);
        }
        if filter.excludes_modifier.intersection(modifier) != WeaponModifierSet::new() {
            return Ok(false);
        }
        if value < filter.min_value {
            if !filter.include_crit_rng_hp {
                return Ok(false);
            }
            if filter.min_value - value > 12 {
                return Ok(false);
            }
            // if within 12, it's possible to crit to the min value
            let result = pot.cook_id(self.recipe_id)?;
            if !result.crit_rng_hp {
                return Ok(false);
            }
            let value = if result.data.effect_id == CookEffect::LifeMaxUp.game_repr_f32() {
                (value + 4).min(112)
            } else {
                (value + 12).min(120)
            };
            if value < filter.min_value {
                return Ok(false);
            }
        }
        if !filter.include_pe_only {
            let mut groups = [Group::None; num_ingr!()];
            if !GroupMnr::<{num_ingr!()}>::new().to_groups(self.recipe_id, &mut groups) {
                // invalid ID - shouldn't happen
                return Ok(false);
            }
            for group in groups {
                if group.all_pe_only() {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }
}

/// A record in the compact DB
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Record(u16);

impl From<u16> for Record {
    #[inline]
    fn from(value: u16) -> Self {
        Record(value)
    }
}

impl From<Record> for u16 {
    #[inline]
    fn from(record: Record) -> Self {
        record.0
    }
}

impl Record {
    pub fn from_data(data: &CookData, crit_rng_hp: bool) -> Self {
        let mut hp = data.health_recover;
        if data.crit_chance >= 100 && !crit_rng_hp {
            // guaranteed crit but no heart rng crit, which means guaranteed heart crit
            if data.effect_id == CookEffect::LifeMaxUp.game_repr_f32() {
                // hearty adds 4
                // technically this should go out of 112, because it's 108 + 4
                // (max is 108 but game doesn't check the cap when crit)
                hp += 4;
            } else {
                hp = (hp + 12).min(120);
            }
        }
        let price = data.sell_price;
        // hhhh hhhp pppp pppp
        let record = (hp << 9) as u16 | (price & 0x1FF) as u16;
        Self(record)
    }
    #[inline]
    pub fn value(self) -> i32 {
        (self.0 >> 9).into()
    }
    #[inline]
    pub fn modifier(self) -> u32 {
        (self.0 as u32) & 0x1FF
    }
    #[inline]
    pub fn modifier_set(self) -> WeaponModifierSet {
        self.modifier().into()
    }

    /// Create a record from raw slice read from the database
    pub fn from_slice(slice: &[u8]) -> Self {
        u16::from_le_bytes([slice[0], slice[1]]).into()
    }

    /// Write the record to a writer
    pub fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.0.to_le_bytes())
    }
}

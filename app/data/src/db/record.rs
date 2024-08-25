use crate::{
    cook::{CookData, CookEffect, CookingPot},
    recipe::{RecipeId, RecipeInputs},
    wmc::WeaponModifierSet,
};

use super::{Error, Filter};

pub struct PositionedRecord {
    pub recipe_id: RecipeId,
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
            let result = pot.cook_inputs(self.recipe_id)?;
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
            let inputs: RecipeInputs = self.recipe_id.into();
            for group in inputs.as_slice() {
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
        ((self.0 as u32) & 0x1FF).into()
    }
    #[inline]
    pub fn modifier_set(self) -> WeaponModifierSet {
        self.modifier().into()
    }
}

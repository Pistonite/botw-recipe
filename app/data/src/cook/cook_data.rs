use serde::{Deserialize, Serialize};
use std::io;

use botw_recipe_generated::CookEffect;

/// This data mirrors uking::ui::PouchItem::CookData, with an extra crit_chance field
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct CookData {
    /// Number of quarter-hearts.
    ///
    /// For non-hearty food, this is usually 0-120.
    /// For hearty food, it's the number of yellow quarter-hearts.
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
    pub crit_chance: i32,
}

impl Default for CookData {
    fn default() -> Self {
        Self::new()
    }
}

impl CookData {
    pub fn new() -> Self {
        Self {
            health_recover: 0,
            effect_duration: 0,
            sell_price: 0,
            effect_id: -1.0,
            effect_level: 0.,
            crit_chance: 0,
        }
    }
    /// Return an invalid CookData with all 0 bytes
    pub fn invalid() -> Self {
        Self {
            health_recover: 0,
            effect_duration: 0,
            sell_price: 0,
            effect_id: 0.,
            effect_level: 0.,
            crit_chance: 0,
        }
    }

    pub fn write_to<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
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
            crit_chance,
        })
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
            return Some(CookDataInvalidReason::InvalidHealthRecover(
                self.health_recover,
            ));
        }
        if self.effect_duration < 0 || self.effect_duration > 1800 {
            return Some(CookDataInvalidReason::InvalidEffectDuration(
                self.effect_duration,
            ));
        }
        if self.sell_price < 0 {
            return Some(CookDataInvalidReason::NegativeSellPrice(self.sell_price));
        }
        if self.effect_level < 0.0 {
            return Some(CookDataInvalidReason::NegativeEffectLevel(
                self.effect_level,
            ));
        }
        if self.effect_level.round() != self.effect_level {
            return Some(CookDataInvalidReason::NonIntegerEffectLevel(
                self.effect_level,
            ));
        }
        let effect = match CookEffect::from_game_repr(self.effect_id) {
            Some(v) => v,
            None => return Some(CookDataInvalidReason::UnknownEffect(self.effect_id)),
        };
        match effect {
            CookEffect::None => {}
            CookEffect::LifeRecover => {
                //supposed to be unused
                return Some(CookDataInvalidReason::LifeRecover);
            }
            CookEffect::LifeMaxUp => {
                if self.effect_level > 100.0 {
                    // max is 5 big hearty radish which gives 25 hearts, or 100 quarter hearts
                    // note that hearty cap is 108 because you have 3 hearts by default
                    return Some(CookDataInvalidReason::LifeMaxUpTooHigh(self.effect_level));
                }
            }
            CookEffect::ResistHot | CookEffect::ResistCold | CookEffect::Fireproof => {
                if self.effect_level > 2.0 {
                    return Some(CookDataInvalidReason::EffectLevelTooHigh(self.effect_level));
                }
            }
            CookEffect::ResistElectric
            | CookEffect::AttackUp
            | CookEffect::DefenseUp
            | CookEffect::Quietness
            | CookEffect::MovingSpeed => {
                if self.effect_level > 3.0 {
                    return Some(CookDataInvalidReason::EffectLevelTooHigh(self.effect_level));
                }
            }
            CookEffect::GutsRecover => {
                // 3000 is 3 wheels
                if self.effect_level > 3000.0 {
                    return Some(CookDataInvalidReason::EffectLevelTooHigh(self.effect_level));
                }
            }
            CookEffect::ExGutsMaxUp => {
                // 10 is 2 wheels, theoretically it's 3 wheels max
                // but you can't get over 2 wheels in game
                if self.effect_level > 10.0 {
                    return Some(CookDataInvalidReason::EffectLevelTooHigh(self.effect_level));
                }
            }
        }

        None
    }
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
    InvalidNoneEffect { sell_price: i32, effect_level: f32 },
    LifeRecover,
    LifeMaxUpTooHigh(f32),
    EffectLevelTooHigh(f32),
    UnknownEffect(f32),
}

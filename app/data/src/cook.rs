//! Cooking-related structs and enums, with utils to convert between 
//! representations in game and in this library

use std::io;

use serde::{Deserialize, Serialize};

pub use crate::generated::CookItem;

// use crate::wmc::WeaponData;

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
    pub crit_chance: i32
}

impl CookData {
    pub fn new() -> Self {
        Self {
            health_recover: 0,
            effect_duration: 0,
            sell_price: 0,
            effect_id: -1.0,
            effect_level: 0.,
            crit_chance: 0
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
            crit_chance: 0
        }
    }

    // pub fn as_weapon(&self) -> &WeaponData {
    //     unsafe {
    //         std::mem::transmute(self)
    //     }
    // }

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
            crit_chance
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
        let effect = match CookEffect::from_game_repr(self.effect_id) {
            Some(v) => v,
            None => return Some(CookDataInvalidReason::UnknownEffect(self.effect_id))
        };
        match effect {
            CookEffect::None => {
                // don't check for now
                // if self.effect_level != 1.0 {
                //     // must be fairy tunic
                //     if self.sell_price != 2 {
                //         return Some(CookDataInvalidReason::InvalidNoneEffect {
                //             sell_price: self.sell_price,
                //             effect_level: self.effect_level
                //         });
                //     }
                // }
            }
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
                if self.effect_level> 2.0 {
                    return Some(CookDataInvalidReason::EffectLevelTooHigh(self.effect_level));
                }
            }
            CookEffect::ResistElectric |
            CookEffect::AttackUp | CookEffect::DefenseUp | CookEffect::Quietness | CookEffect::MovingSpeed
            => {
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
    InvalidNoneEffect {
        sell_price: i32,
        effect_level: f32
    },
    LifeRecover,
    LifeMaxUpTooHigh(f32),
    EffectLevelTooHigh(f32),
    UnknownEffect(f32),
}

/// Cook modifier that can be converted to uking::CookingMgr::CookEffect
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash, Default, Deserialize)]
pub enum CookEffect {
    AttackUp,
    DefenseUp,
    ResistCold,
    ResistHot,
    ResistElectric,
    Fireproof,
    MovingSpeed,
    Quietness,
    LifeMaxUp,
    GutsRecover,
    ExGutsMaxUp,
    LifeRecover, // Unused
    #[default]
    None,
}

impl CookEffect {
    /// Convert to enum values used in game
    pub const fn game_repr(self) -> i32 {
        match self {
            Self::AttackUp => 10,
            Self::DefenseUp => 11,
            Self::ResistCold => 5,
            Self::ResistHot => 4,
            Self::ResistElectric => 6,
            Self::Fireproof => 16,
            Self::MovingSpeed => 13,
            Self::Quietness => 12,
            Self::LifeMaxUp => 2,
            Self::GutsRecover => 14,
            Self::ExGutsMaxUp => 15,
            Self::LifeRecover => 1,
            Self::None => -1,
        }
    }

    pub fn from_game_repr(v: f32) -> Option<Self> {
        match v{
            -1. => Some(Self::None),
            1. => Some(Self::LifeRecover),
            2. => Some(Self::LifeMaxUp),
            4. => Some(Self::ResistHot),
            5. => Some(Self::ResistCold),
            6. => Some(Self::ResistElectric),
            10. => Some(Self::AttackUp),
            11. => Some(Self::DefenseUp),
            12. => Some(Self::Quietness),
            13. => Some(Self::MovingSpeed),
            14. => Some(Self::GutsRecover),
            15. => Some(Self::ExGutsMaxUp),
            16. => Some(Self::Fireproof),
            _ => None,
        }
    }

    /// Convert to enum values used in game
    pub fn game_repr_f32(self) -> f32 {
        self.game_repr() as f32
    }

    /// Get the data associated with this effect
    pub fn data(self) -> &'static CookEffectData {
        match self {
            CookEffect::None => &NONE,
            CookEffect::LifeRecover => &LIFE_RECOVER,
            CookEffect::LifeMaxUp => &LIFE_MAX_UP,
            CookEffect::ResistHot => &RESIST_HOT,
            CookEffect::ResistCold => &RESIST_COLD,
            CookEffect::ResistElectric => &RESIST_ELECTRIC,
            CookEffect::AttackUp => &ATTACK_UP,
            CookEffect::DefenseUp => &DEFENSE_UP,
            CookEffect::Quietness => &QUIETNESS,
            CookEffect::MovingSpeed => &MOVING_SPEED,
            CookEffect::GutsRecover => &GUTS_RECOVER,
            CookEffect::ExGutsMaxUp => &EX_GUTS_MAX_UP,
            CookEffect::Fireproof => &FIREPROOF,
        }
    }

    /// If effect_time is computed for this effect
    pub fn uses_time(self) -> bool {
        self.data().base_time != 0
    }

}

impl std::fmt::Display for CookEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Data for a [`CookEffect`] used to compute effect duration, level, etc
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct CookEffectData {
    /// The base effect time when the cooked result have this effect
    /// 0 indicates that the cook effect doesn't use time, which
    /// is important when computing the effect duration
    pub base_time: i32,
    /// The maximum level of the effect
    pub max_level: i32,

    /// The minimum potency needed for LV2 effect
    pub potency_lv2: i32,
    /// The minimum potency needed for LV3 effect
    pub potency_lv3: i32,
    /// English name of the effect
    pub name: &'static str,
}

static NONE: CookEffectData = CookEffectData {
    base_time: 0, 
    max_level: 0, // unused
    potency_lv2: -1,
    potency_lv3: -1,
    name: "",
};

static EX_GUTS_MAX_UP: CookEffectData = CookEffectData {
    base_time: 0,
    max_level: 15, 
    potency_lv2: -1,
    potency_lv3: -1,
    name: "Enduring",
};

static GUTS_RECOVER: CookEffectData = CookEffectData {
    base_time: 0, 
    max_level: 3000,
    potency_lv2: -1,
    potency_lv3: -1,
    name: "Energizing",
};

static LIFE_RECOVER: CookEffectData = CookEffectData {
    base_time: 0, 
    max_level: 120,
    potency_lv2: -1,
    potency_lv3: -1,
    name: "",
};

static LIFE_MAX_UP: CookEffectData = CookEffectData {
    base_time: 0, 
    max_level: 108,
    potency_lv2: -1,
    potency_lv3: -1,
    name: "Hearty",
};

static RESIST_HOT: CookEffectData = CookEffectData {
    base_time: 120, 
    max_level: 2,
    potency_lv2: 6,
    potency_lv3: 999,
    name: "Chilly",
};

static RESIST_COLD: CookEffectData = CookEffectData {
    base_time: 120, 
    max_level: 2,
    potency_lv2: 6,
    potency_lv3: 999,
    name: "Spicy",
};

static RESIST_ELECTRIC: CookEffectData = CookEffectData {
    base_time: 120, 
    max_level: 3,
    potency_lv2: 4,
    potency_lv3: 6,
    name: "Electro",
        };
static MOVING_SPEED: CookEffectData = CookEffectData {
    base_time: 30, 
    max_level: 3,
    potency_lv2: 5,
    potency_lv3: 7,
    name: "Hasty",
};

static ATTACK_UP: CookEffectData = CookEffectData {
    base_time: 20, 
    max_level: 3,
    potency_lv2: 5,
    potency_lv3: 7,
    name: "Mighty",
};

static DEFENSE_UP: CookEffectData = CookEffectData {
    base_time: 20, 
    max_level: 3,
    potency_lv2: 5,
    potency_lv3: 7,
    name: "Tough",
};

static QUIETNESS: CookEffectData = CookEffectData {
    base_time: 90, 
    max_level: 3,
    potency_lv2: 6,
    potency_lv3: 9,
    name: "Sneaky",
};

static FIREPROOF: CookEffectData = CookEffectData {
    base_time: 120, 
    max_level: 2,
    potency_lv2: 7,
    potency_lv3: 999,
    name: "Fireproof",
};

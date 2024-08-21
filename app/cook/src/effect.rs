use rdata::cook::CookEffect;
use serde::{Deserialize, Serialize};

use crate::Error;

/// Data for a [`CookEffect`] used to compute effect duration, level, etc
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct CookEffectData {
    /// The base effect time when the cooked result have this effect
    pub base_time: i32,
    /// The minimum potency needed for LV2 effect
    pub potency_lv2: i32,
    /// The minimum potency needed for LV3 effect
    pub potency_lv3: i32,
    /// English name of the effect
    pub name: &'static str,
    max: i32,
    min: i32,
    ssa: i32,
    /// The associated cook effect
    kind: CookEffect,
}

impl TryFrom<CookEffect> for CookEffectData {
    type Error = Error;
    fn try_from(value: CookEffect) -> Result<Self, Error> {
        Ok(match value {
            CookEffect::None => return Err(Error::NoEffectData),
            CookEffect::LifeRecover => LIFE_RECOVER,
            CookEffect::LifeMaxUp => LIFE_MAX_UP,
            CookEffect::ResistHot => RESIST_HOT,
            CookEffect::ResistCold => RESIST_COLD,
            CookEffect::ResistElectric => RESIST_ELECTRIC,
            CookEffect::AttackUp => ATTACK_UP,
            CookEffect::DefenseUp => DEFENSE_UP,
            CookEffect::Quietness => QUIETNESS,
            CookEffect::MovingSpeed => MOVING_SPEED,
            CookEffect::GutsRecover => GUTS_RECOVER,
            CookEffect::ExGutsMaxUp => EX_GUTS_MAX_UP,
            CookEffect::Fireproof => FIREPROOF,
        })
    }
}

static EX_GUTS_MAX_UP: CookEffectData = CookEffectData {
    base_time: 0, 
    potency_lv2: -1,
    potency_lv3: -1,
    name: "Enduring",
    max: 15,
    min: 1,
    ssa: 2,
    kind: CookEffect::ExGutsMaxUp,
};

static GUTS_RECOVER: CookEffectData = CookEffectData {
    base_time: 0, 
    potency_lv2: -1,
    potency_lv3: -1,
    name: "Energizing",
    max: 20,
    min: 1,
    ssa: 2,
    kind: CookEffect::GutsRecover,
};

static LIFE_RECOVER: CookEffectData = CookEffectData {
    base_time: 0, 
    potency_lv2: -1,
    potency_lv3: -1,
    name: "",
    max: 120,
    min: 1,
    ssa: 12,
    kind: CookEffect::LifeRecover,
};

static LIFE_MAX_UP: CookEffectData = CookEffectData {
    base_time: 0, 
    potency_lv2: -1,
    potency_lv3: -1,
    name: "Hearty",
    max: 108,
    min: 4,
    ssa: 4,
    kind: CookEffect::LifeMaxUp,
};

static RESIST_HOT: CookEffectData = CookEffectData {
    base_time: 120, 
    potency_lv2: 6,
    potency_lv3: 999,
    name: "Chilly",
    max: 2,
    min: 1,
    ssa: 1,
    kind: CookEffect::ResistHot,
};

static RESIST_COLD: CookEffectData = CookEffectData {
    base_time: 120, 
    potency_lv2: 6,
    potency_lv3: 999,
    name: "Spicy",
    max: 2,
    min: 1,
    ssa: 1,
    kind: CookEffect::ResistCold,
};

static RESIST_ELECTRIC: CookEffectData = CookEffectData {
    base_time: 120, 
    potency_lv2: 4,
    potency_lv3: 6,
    name: "Electro",
    max: 3,
    min: 1,
    ssa: 1,
    kind: CookEffect::ResistElectric,
        };
static MOVING_SPEED: CookEffectData = CookEffectData {
    base_time: 30, 
    potency_lv2: 5,
    potency_lv3: 7,
    name: "Hasty",
    max: 3,
    min: 1,
    ssa: 1,
    kind: CookEffect::MovingSpeed,
};

static ATTACK_UP: CookEffectData = CookEffectData {
    base_time: 20, 
    potency_lv2: 5,
    potency_lv3: 7,
    name: "Mighty",
    max: 3,
    min: 1,
    ssa: 1,
    kind: CookEffect::AttackUp,
};

static DEFENSE_UP: CookEffectData = CookEffectData {
    base_time: 20, 
    potency_lv2: 5,
    potency_lv3: 7,
    name: "Tough",
    max: 3,
    min: 1,
    ssa: 1,
    kind: CookEffect::DefenseUp,
};

static QUIETNESS: CookEffectData = CookEffectData {
    base_time: 90, 
    potency_lv2: 6,
    potency_lv3: 9,
    name: "Sneaky",
    max: 3,
    min: 1,
    ssa: 1,
    kind: CookEffect::Quietness,
};

static FIREPROOF: CookEffectData = CookEffectData {
    base_time: 120, 
    potency_lv2: 7,
    potency_lv3: 999,
    name: "Fireproof",
    max: 2,
    min: 1,
    ssa: 1,
    kind: CookEffect::Fireproof,
};

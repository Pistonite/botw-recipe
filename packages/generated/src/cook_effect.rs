use serde::{Deserialize, Serialize};

/// Cook modifier that can be converted to uking::CookingMgr::CookEffect
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash, Default, Deserialize)]
#[repr(u8)]
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
        match v {
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

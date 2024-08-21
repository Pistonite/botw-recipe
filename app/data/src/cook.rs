//! Cooking-related structs and enums, with utils to convert between 
//! representations in game and in this library

use std::io;

use serde::{Deserialize, Serialize};

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

    pub fn as_weapon(&self) -> &WeaponData {
        unsafe {
            std::mem::transmute(self)
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

/// Weapon modifier data
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct WeaponData {
    pub modifier_value: u32,
    unused: u32,
    pub modifier_bitset: u32,
    unused2: f32,
    unused3: f32,
    crit_chance: i32
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

    /// If effect_time is computed for this effect
    pub const fn uses_time(self) -> bool {
        match self {
            Self::None | Self::LifeMaxUp | Self::GutsRecover | Self::ExGutsMaxUp => false,
            _ => true,
        }
    }

    /// Whether potency is used to calculate effect level
    pub const fn uses_potency(self) -> bool {
        match self {
            Self::None | Self::LifeRecover | Self::LifeMaxUp | Self::GutsRecover | Self::ExGutsMaxUp => false,
            _ => true,
        }
    }
}

impl std::fmt::Display for CookEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(u32)]
pub enum WeaponModifier{
    None = 0,
    /// Attack up for swords/bows/shields (lynel)
    AddAtk = 0x1,
    /// Durability up
    AddLife = 0x2,
    /// Critical hit (sword)
    AddCrit = 0x4,
    /// Long throw (sword)
    AddThrow = 0x8,
    /// Multishot (bow)
    ///
    /// Multishot will be spread fire, and unless the bow is originally
    /// multishot, the spread will be very big. Multishot + Zoom
    /// will be focus shot instead of spread
    AddSpreadFire = 0x10,
    /// Zoom (bow). 
    AddZoom = 0x20,
    /// Quick shot (bow)
    AddRapidFire = 0x40,
    /// Slick shield
    AddSurfMaster = 0x80,
    /// Guard up (shield)
    AddGuard = 0x100,
    /// Yellow modifier
    IsYellow = 0x80000000,
}

impl std::ops::BitOr for WeaponModifier {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        unsafe { std::mem::transmute(self as u32 | rhs as u32) }
    }
}

impl std::ops::BitAnd for WeaponModifier {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        unsafe { std::mem::transmute(self as u32 & rhs as u32) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WeaponModifierInfo {
    /// Attack up +X
    attack_up: Option<u32>,
    /// Durability up (value is added directly in inventory, not here)
    durability_up: bool,
    /// Critical hit
    critical_hit: bool,
    /// Long throw. Value is hp/1000 (max 0.12), so it will become "Short throw"
    long_throw: Option<f32>,
    /// Multishot. Value is number of arrows. The value here will be whatever the hp
    /// is before transmuted, but the game will cap it at 10
    ///
    /// Note that the frame rate and bow fire speed will sometimes not allow 10 to be fired
    multi_shot: Option<u32>,
    /// Zoom
    zoom: bool,
    /// Quick shot. Value is hp/1000 (max 0.12), so it will be "Slow shot"
    quick_shot: Option<f32>,
    /// Surf master. Value is friction and max is hp/1000 (max 0.12), so it will be slicky
    surf_master: Option<f32>,
    /// Shield guard up +X
    shield_guard_up: Option<u32>,
    /// Yellow modifier
    is_yellow: bool,
}

impl<W: AsRef<WeaponData>> From<W> for WeaponModifierInfo {
    fn from(value: W) -> Self {
        let value = value.as_ref();
let m = value.modifier_bitset;
        let v = value.modifier_value;
        let attack_up = (m & WeaponModifier::AddAtk as u32 !=0).then_some(v);
        let durability_up = m & WeaponModifier::AddLife as u32 != 0;
        let critical_hit = m & WeaponModifier::AddCrit as u32 != 0;
        let long_throw = (m & WeaponModifier::AddThrow as u32!=0).then_some(v as f32 / 1000.);
        let multi_shot = (m & WeaponModifier::AddSpreadFire as u32!=0).then_some(v);
        let zoom = m & WeaponModifier::AddZoom as u32 != 0;
        let quick_shot = (m & WeaponModifier::AddRapidFire as u32!=0).then_some(v as f32 / 1000.);
        let surf_master = (m & WeaponModifier::AddSurfMaster as u32!=0).then_some(v as f32 / 1000.);
        let shield_guard_up = (m & WeaponModifier::AddGuard as u32!=0).then_some(v);
        let is_yellow = m & WeaponModifier::IsYellow as u32 != 0;
        Self {
            attack_up,
            durability_up,
            critical_hit,
            long_throw,
            multi_shot,
            zoom,
            quick_shot,
            surf_master,
            shield_guard_up,
            is_yellow,
        }
    }
}

impl WeaponModifierInfo {
    // pub fn has(modifier: WeaponModifier) -> bool {
    // }
}

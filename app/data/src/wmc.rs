use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum WeaponModifier {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(transparent)]
pub struct WeaponModifierSet(u32);

impl From<u32> for WeaponModifierSet {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<WeaponModifierSet> for u32 {
    #[inline]
    fn from(value: WeaponModifierSet) -> Self {
        value.0
    }
}

impl WeaponModifierSet {
    #[inline]
    pub fn new() -> Self {
        Self(WeaponModifier::None as u32)
    }
    #[inline]
    pub fn all() -> Self {
        (0x1FF | WeaponModifier::IsYellow as u32).into()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == WeaponModifier::None as u32
    }
    #[inline]
    pub fn has(&self, modifier: WeaponModifier) -> bool {
        self.0 & modifier as u32 != 0
    }
    #[inline]
    pub fn add(&mut self, modifier: WeaponModifier) {
        self.0 |= modifier as u32;
    }
    #[inline]
    pub fn remove(&mut self, modifier: WeaponModifier) {
        self.0 &= !(modifier as u32);
    }
    #[inline]
    pub fn union<T: Into<Self>>(&self, other: T) -> Self {
        Self(self.0 | other.into().0)
    }
    #[inline]
    pub fn intersection<T: Into<Self>>(&self, other: T) -> Self {
        Self(self.0 & other.into().0)
    }
    #[inline]
    pub fn compliment(&self) -> Self {
        Self(!self.0).intersection(Self::all())
    }
}

#[macro_export]
macro_rules! weapon_modifier_set {
    ($($modifier:ident)|*) => {
        {
            let mut set = WeaponModifierSet::new();
            $(
                set.add(WeaponModifier::$modifier);
            )*
            set
        }
    };
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

// impl<W: AsRef<WeaponData>> From<W> for WeaponModifierInfo {
//     fn from(value: W) -> Self {
//         let value = value.as_ref();
// let m = value.modifier_bitset;
//         let v = value.modifier_value;
//         let attack_up = (m & WeaponModifier::AddAtk as u32 !=0).then_some(v);
//         let durability_up = m & WeaponModifier::AddLife as u32 != 0;
//         let critical_hit = m & WeaponModifier::AddCrit as u32 != 0;
//         let long_throw = (m & WeaponModifier::AddThrow as u32!=0).then_some(v as f32 / 1000.);
//         let multi_shot = (m & WeaponModifier::AddSpreadFire as u32!=0).then_some(v);
//         let zoom = m & WeaponModifier::AddZoom as u32 != 0;
//         let quick_shot = (m & WeaponModifier::AddRapidFire as u32!=0).then_some(v as f32 / 1000.);
//         let surf_master = (m & WeaponModifier::AddSurfMaster as u32!=0).then_some(v as f32 / 1000.);
//         let shield_guard_up = (m & WeaponModifier::AddGuard as u32!=0).then_some(v);
//         let is_yellow = m & WeaponModifier::IsYellow as u32 != 0;
//         Self {
//             attack_up,
//             durability_up,
//             critical_hit,
//             long_throw,
//             multi_shot,
//             zoom,
//             quick_shot,
//             surf_master,
//             shield_guard_up,
//             is_yellow,
//         }
//     }
// }

impl WeaponModifierInfo {
    // pub fn has(modifier: WeaponModifier) -> bool {
    // }
}

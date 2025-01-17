use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum WeaponModifier {
    None = 0,
    /// Attack up for swords/bows/shields (lynel)
    AddPower = 0x1,
    /// Durability up
    AddLife = 0x2,
    /// Critical hit (sword)
    Critical = 0x4,
    /// Long throw (sword)
    AddThrow = 0x8,
    /// Multishot (bow)
    ///
    /// Multishot will be spread fire, and unless the bow is originally
    /// multishot, the spread will be very big. Multishot + Zoom
    /// will be focus shot instead of spread
    SpreadFire = 0x10,
    /// Zoom (bow).
    Zoom = 0x20,
    /// Quick shot (bow)
    RapidFire = 0x40,
    /// Slick shield
    SurfMaster = 0x80,
    /// Guard up (shield)
    AddGuard = 0x100,
    /// Yellow modifier
    IsYellow = 0x80000000,
}

#[derive(Debug, Derivative, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derivative(Default(new = "true"))]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
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


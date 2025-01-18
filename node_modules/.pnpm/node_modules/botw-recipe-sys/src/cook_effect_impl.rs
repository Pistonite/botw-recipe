use super::CookEffect;

impl CookEffect {
    /// Convert the CookEffect to an integer representation
    ///
    /// Note this does not have any meaning in the Game,
    /// and it is not guaranteed to be the same as EnumMap/EnumSet
    /// implementation
    #[inline]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    #[inline]
    pub const fn from_u8(v: u8) -> Option<Self> {
        if v < crate::cook_effect_count!() {
            Some(unsafe { std::mem::transmute(v) })
        } else {
            None
        }
    }

    /// If effect_time is computed for this effect
    #[inline]
    #[cfg(feature = "cook-effect-data")]
    pub const fn uses_time(self) -> bool {
        self.base_time() != 0
    }

    /// Get base_time as i32
    #[inline]
    #[cfg(feature = "cook-effect-data")]
    pub const fn base_time_i32(self) -> i32 {
        self.base_time() as i32
    }

    /// Get the game representation of the effect as f32
    #[inline]
    pub const fn game_repr_f32(self) -> f32 {
        self.game_repr() as f32
    }
    /// Get the potency thresholds for getting Lv2 and Lv3 of the effect
    ///
    /// If the effect only has Lv2, then the Lv3 threshold is 999.
    ///
    /// For non-time-based effects, returns (-1, -1)
    #[cfg(feature = "cook-effect-data")]
    pub const fn get_potency_thresholds(self) -> (i32, i32) {
        match self {
            CookEffect::AttackUp => (5, 7),
            CookEffect::DefenseUp => (5, 7),
            CookEffect::ResistCold => (6, 999),
            CookEffect::ResistHot => (6, 999),
            CookEffect::ResistElectric => (4, 6),
            CookEffect::Fireproof => (7, 999),
            CookEffect::MovingSpeed => (5, 7),
            CookEffect::Quietness => (6, 9),
            CookEffect::LifeMaxUp => (-1, -1),
            CookEffect::GutsRecover => (-1, -1),
            CookEffect::ExGutsMaxUp => (-1, -1),
            CookEffect::LifeRecover => (-1, -1),
            CookEffect::None => (-1, -1),
        }
    }
}

#[cfg(feature = "cook-effect-to-str")]
impl std::fmt::Debug for CookEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

#[cfg(not(feature = "cook-effect-to-str"))]
impl std::fmt::Debug for CookEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_u8().fmt(f)
    }
}

#[cfg(feature = "cook-effect-to-str")]
impl std::fmt::Display for CookEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

#[cfg(feature = "cook-effect-serde-serialize")]
impl serde::Serialize for CookEffect {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_str().serialize(serializer)
    }
}

#[cfg(feature = "cook-effect-serde-deserialize")]
impl<'de> serde::Deserialize<'de> for CookEffect {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        struct Visitor;
        impl<'d> serde::de::Visitor<'d> for Visitor {
            type Value=CookEffect;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                use std::fmt::Display;
                "a valid cook effect name".fmt(formatter)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match CookEffect::from_str(v) {
                    Some(item) => Ok(item),
                    None => Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &self)),
                }
            }

        }
        d.deserialize_str(Visitor)
    }
}

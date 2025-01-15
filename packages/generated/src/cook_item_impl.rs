use crate::CookItem;

impl CookItem {
    /// Convert the CookItem to an integer representation
    ///
    /// Note this does not have any meaning in the Game,
    /// and it is not guaranteed to be the same as EnumMap/EnumSet
    /// implementation
    #[inline]
    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    #[inline]
    pub const fn from_u8(v: u8) -> Option<Self> {
        if v < crate::cook_item_count!() {
            Some(unsafe { std::mem::transmute(v) })
        } else {
            None
        }
    }

    /// Get the Rock-hard food CookItem
    #[inline] 
    pub const fn rock_hard() -> Self {
        Self::Item_Cook_O_02
    }

    /// Check if this CookItem is a Rock-hard food
    #[inline] 
    pub fn is_rock_hard(self) -> bool {
        self == Self::rock_hard()
    }

    /// Get the elixir CookItem
    #[inline]
    pub const fn elixir() -> Self {
        Self::Item_Cook_C_17
    }

    /// Check if this CookItem is an Elixir
    #[inline]
    pub fn is_elixir(self) -> bool {
        self == Self::elixir()
    }

    /// Check if this CookItem is a Dubious Food
    #[inline]
    pub fn is_dubious(self) -> bool {
        self == Self::dubious_food()
    }

    /// Check if this CookItem is a Fairy Tonic
    #[inline]
    pub fn is_fairy_tonic(self) -> bool {
        self == Self::fairy_tonic()
    }


}

#[cfg(all(feature = "cook-item-english", feature = "cook-item-to-actor"))]
impl std::fmt::Debug for CookItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(self.actor_name())
            .field(&self.name())
            .finish()
    }
}

#[cfg(all(not(feature = "cook-item-english"), feature = "cook-item-to-actor"))]
impl std::fmt::Debug for CookItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.actor_name().fmt(f)
    }
}

#[cfg(all(not(feature = "cook-item-english"), not(feature = "cook-item-to-actor")))]
impl std::fmt::Debug for CookItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_u8().fmt(f)
    }
}

#[cfg(feature = "cook-item-english")]
impl std::fmt::Display for CookItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name().fmt(f)
    }
}

#[cfg(all(not(feature = "cook-item-english"), feature = "cook-item-to-actor"))]
impl std::fmt::Display for CookItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.actor_name().fmt(f)
    }
}

#[cfg(all(not(feature = "cook-item-english"), not(feature = "cook-item-to-actor")))]
impl std::fmt::Display for CookItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_u8().fmt(f)
    }
}

#[cfg(feature = "cook-item-serde-serialize-value")]
impl serde::Serialize for CookItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_u8().serialize(serializer)
    }
}

#[cfg(feature = "cook-item-serde-serialize-actor")]
impl serde::Serialize for CookItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.actor_name().serialize(serializer)
    }
}

#[cfg(any(feature = "cook-item-serde-deserialize-actor", feature = "cook-item-serde-deserialize-value"))]
impl<'de> serde::Deserialize<'de> for CookItem {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        struct Visitor;
        impl<'d> serde::de::Visitor<'d> for Visitor {
            type Value=CookItem;

            #[cfg(all(feature = "cook-item-serde-deserialize-actor", not(feature = "cook-item-serde-deserialize-value")))]
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                use std::fmt::Display;
                "a valid cook item actor name starting with Item_Cook_".fmt(formatter)
            }

            #[cfg(all(not(feature = "cook-item-serde-deserialize-actor"), feature = "cook-item-serde-deserialize-value"))]
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                use std::fmt::Display;
                "a valid integer representing a CookItem enum".fmt(formatter)
            }

            #[cfg(all(feature = "cook-item-serde-deserialize-actor", feature = "cook-item-serde-deserialize-value"))]
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                use std::fmt::Display;
                "a valid integer representing a CookItem enum, or a valid cook item actor name starting with Item_Cook_".fmt(formatter)
            }

            fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u8(v as u8)
            }

            fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u8(v as u8)
            }

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u8(v as u8)
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u8(v as u8)
            }

            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u8(v as u8)
            }

            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match CookItem::from_u8(v) {
                    Some(item) => Ok(item),
                    None => Err(serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v as u64), &self)),
                }
            }

            fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u8(v as u8)
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u8(v as u8)
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u8(v as u8)
            }

            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u8(v as u8)
            }

            #[cfg(feature = "cook-item-serde-deserialize-actor")]
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match CookItem::from_actor_name(v) {
                    Some(item) => Ok(item),
                    None => Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &self)),
                }
            }

        }
        d.deserialize_str(Visitor)
    }
}

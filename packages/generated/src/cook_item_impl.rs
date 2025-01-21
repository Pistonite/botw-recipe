use crate::CookItem;

impl CookItem {
    /// Convert the CookItem to an integer representation
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

    /// Check if the CookItem is from a recipe with monster extract
    #[inline]
    pub fn is_monster_food(self) -> bool {
            matches!(self, CookItem::Item_Cook_L_01 | CookItem::Item_Cook_L_02 | CookItem::Item_Cook_L_03
| CookItem::Item_Cook_L_04 | CookItem::Item_Cook_L_05)
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

#[cfg(all(feature = "cook-item-english", not(feature = "cook-item-to-actor")))]
impl std::fmt::Debug for CookItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name().fmt(f)
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

#[cfg(feature = "cook-item-serde-serialize")]
impl serde::Serialize for CookItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.actor_name().serialize(serializer)
    }
}

#[cfg(feature = "cook-item-serde-deserialize")]
impl<'de> serde::Deserialize<'de> for CookItem {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        struct Visitor;
        impl<'d> serde::de::Visitor<'d> for Visitor {
            type Value=CookItem;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                use std::fmt::Display;
                "a valid cook item actor name starting with Item_Cook_".fmt(formatter)
            }

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

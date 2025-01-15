use crate::Tag;

impl Tag {
    /// Convert the Tag to an integer representation
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
        if v < crate::tag_count!() {
            Some(unsafe { std::mem::transmute(v) })
        } else {
            None
        }
    }
}

#[cfg(feature = "recipe")]
impl crate::recipe::RecipeMatch for Tag {}

#[cfg(feature = "tag-to-str")]
impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

#[cfg(not(feature = "tag-to-str"))]
impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_u8().fmt(f)
    }
}

#[cfg(feature = "tag-to-str")]
impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

#[cfg(feature = "tag-serde-serialize-str")]
impl serde::Serialize for Tag {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_str().serialize(serializer)
    }
}

#[cfg(feature = "tag-serde-serialize-value")]
impl serde::Serialize for Tag {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_u8().serialize(serializer)
    }
}

#[cfg(any(feature = "tag-serde-deserialize-str", feature = "tag-serde-deserialize-value"))]
impl<'de> serde::Deserialize<'de> for Tag {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        struct Visitor;
        impl<'d> serde::de::Visitor<'d> for Visitor {
            type Value=Tag;

            #[cfg(all(feature = "tag-serde-deserialize-str", not(feature = "tag-serde-deserialize-value")))]
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                use std::fmt::Display;
                "a valid tag name".fmt(formatter)
            }

            #[cfg(all(not(feature = "tag-serde-deserialize-str"), feature = "tag-serde-deserialize-value"))]
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                use std::fmt::Display;
                "a valid integer representing a Tag enum".fmt(formatter)
            }

            #[cfg(all(feature = "tag-serde-deserialize-str", feature = "tag-serde-deserialize-value"))]
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                use std::fmt::Display;
                "a valid integer representing a Tag enum, or a valid tag name".fmt(formatter)
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
                match Tag::from_u8(v) {
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

            #[cfg(feature = "tag-serde-deserialize-str")]
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match Tag::from_str(v) {
                    Some(item) => Ok(item),
                    None => Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &self)),
                }
            }

        }
        d.deserialize_str(Visitor)
    }
}

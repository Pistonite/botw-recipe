use crate::Actor;

impl Actor {
    /// Convert the Actor to an integer representation
    ///
    /// Note this does not have any meaning in the Game,
    /// and it is not guaranteed to be the same as EnumMap/EnumSet
    /// implementation
    #[inline]
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

#[cfg(all(feature = "actor-english", feature = "actor-to-actor"))]
impl std::fmt::Debug for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(self.actor_name())
            .field(&self.name())
            .finish()
    }
}

#[cfg(all(not(feature = "actor-english"), feature = "actor-to-actor"))]
impl std::fmt::Debug for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.actor_name().fmt(f)
    }
}

#[cfg(all(not(feature = "actor-english"), not(feature = "actor-to-actor")))]
impl std::fmt::Debug for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_u8().fmt(f)
    }
}

#[cfg(feature = "actor-english")]
impl std::fmt::Display for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name().fmt(f)
    }
}

#[cfg(all(not(feature = "actor-english"), feature = "actor-to-actor"))]
impl std::fmt::Display for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.actor_name().fmt(f)
    }
}

#[cfg(all(not(feature = "actor-english"), not(feature = "actor-to-actor")))]
impl std::fmt::Display for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_u8().fmt(f)
    }
}

#[cfg(feature = "actor-serde-serialize-value")]
impl serde::Serialize for Actor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_u8().serialize(serializer)
    }
}

#[cfg(feature = "actor-serde-serialize-actor")]
impl serde::Serialize for Actor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.actor_name().serialize(serializer)
    }
}

#[cfg(any(feature = "actor-serde-deserialize-actor", feature = "actor-serde-deserialize-value"))]
impl<'de> serde::Deserialize<'de> for Actor {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        struct Visitor;
        impl<'d> serde::de::Visitor<'d> for Visitor {
            type Value=Actor;

            #[cfg(all(feature = "actor-serde-deserialize-actor", not(feature = "actor-serde-deserialize-value")))]
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                use std::fmt::Display;
                "a valid recipe input actor name".fmt(formatter)
            }

            #[cfg(all(not(feature = "actor-serde-deserialize-actor"), feature = "actor-serde-deserialize-value"))]
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                use std::fmt::Display;
                "a valid integer representing an Actor enum".fmt(formatter)
            }

            #[cfg(all(feature = "actor-serde-deserialize-actor", feature = "actor-serde-deserialize-value"))]
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                use std::fmt::Display;
                "a valid integer representing an Actor enum, or a valid recipe input actor name".fmt(formatter)
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
                match Actor::from_u8(v) {
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

            #[cfg(feature = "actor-serde-deserialize-actor")]
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match Actor::from_actor_name(v) {
                    Some(item) => Ok(item),
                    None => Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &self)),
                }
            }

        }
        d.deserialize_str(Visitor)
    }
}

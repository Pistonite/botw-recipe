use crate::Actor;

impl Actor {
    /// Convert the Actor to an integer representation
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
        if v < crate::actor_count!() {
            Some(unsafe { std::mem::transmute(v) })
        } else {
            None
        }
    }

    #[inline]
    pub const fn is_monster_extract(self) -> bool {
        false //# TODO
    }
}

#[cfg(feature = "recipe")]
impl crate::recipe::RecipeMatch for Actor {}

#[cfg(all(feature = "actor-english", feature = "actor-to-actor"))]
impl std::fmt::Debug for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(self.actor_name())
            .field(&self.name())
            .finish()
    }
}

#[cfg(all(feature = "actor-english", not(feature = "actor-to-actor")))]
impl std::fmt::Debug for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name().fmt(f)
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

#[cfg(feature = "actor-serde-serialize")]
impl serde::Serialize for Actor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.actor_name().serialize(serializer)
    }
}

#[cfg(feature = "actor-serde-deserialize")]
impl<'de> serde::Deserialize<'de> for Actor {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        struct Visitor;
        impl<'d> serde::de::Visitor<'d> for Visitor {
            type Value=Actor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                use std::fmt::Display;
                "a valid recipe input actor name".fmt(formatter)
            }

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

/// Actor multichoose series
///
/// `R` is the number of actors in the output. Must be <= 5
#[cfg(feature = "multichoose")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ActorMnr<const R: usize={crate::num_ingr!()}> {
    inner: crate::multichoose::Mnr<{crate::actor_count!()}, R>
}

#[cfg(feature = "multichoose")]
impl<const R: usize> ActorMnr<R> {
    pub fn new() -> Self {
        Self { inner: crate::multichoose::Mnr::new() }
    }

    /// Convert serial ID to unique actors
    #[cfg(feature = "actor-enum-set")]
    pub fn to_unique_actors(self, id: u64) -> enumset::EnumSet<Actor> {
        let mut out = enumset::EnumSet::new();
        let mut inner_out = [0u32; R];
        let res = self.inner.serial_to_choices(id, &mut inner_out);
        if !res {
            return out;
        }
        for i in 0..R {
            let Some(actor) = Actor::from_u8(inner_out[i] as u8) else {
                return enumset::EnumSet::new();
            };
            out.insert(actor);
        }

        out
    }

    /// Convert serial ID to the actor choices.
    ///
    /// Returns false if the id is out of bound
    #[must_use]
    pub fn to_actors(self, id: u64, out: &mut [Actor; R]) -> bool {
        let mut inner_out = [0u32; R];
        let res = self.inner.serial_to_choices(id, &mut inner_out);
        if !res {
            return false;
        }
        for i in 0..R {
            let Some(actor) = Actor::from_u8(inner_out[i] as u8) else {
                return false;
            };
            out[i] = actor;
        }

        true
    }

    /// Convert actor choices to serial ID.
    pub fn to_serial(self, actors: &[Actor; R]) -> Option<u64> {
        let mut inner_actors = [0u32; R];
        for i in 0..R {
            inner_actors[i] = actors[i].as_u8() as u32;
        }
        inner_actors.sort_unstable();
        self.inner.choices_to_serial(&inner_actors)
    }

    #[inline]
    pub fn len(self) -> u64 {
        self.inner.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "multichoose")]
    pub fn test_actor_mnr() {
        let mnr = ActorMnr::<5>::new();

        let mut out = [Actor::None; 5];
        assert!(mnr.to_actors(1, &mut out));
        assert_eq!(out, [Actor::None, Actor::None, Actor::None, Actor::None, Actor::Item_Fruit_D]);
    }

    #[test]
    #[cfg(feature = "multichoose")]
    pub fn test_actor_mnr_reverse() {
        let mnr = ActorMnr::<5>::new();

        assert_eq!(mnr.to_serial(&[Actor::None, Actor::None, Actor::None, Actor::None, Actor::Item_Fruit_D]), Some(1));
    }
}

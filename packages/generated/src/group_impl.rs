use crate::{Actor, Group};

impl Group {
    /// Convert the Group to an integer representation
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
        if v < crate::group_count!() {
            Some(unsafe { std::mem::transmute(v) })
        } else {
            None
        }
    }

    /// Get the first actor in the group
    #[inline]
    pub const fn first_actor(self) -> Actor {
        match self {
            Self::None => Actor::None,
            _ => self.actors()[0]
        }
    }
}

/// Actor multichoose series
///
/// `R` is the number of actors in the output. Must be <= 5
#[cfg(feature = "multichoose")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GroupMnr<const R: usize> {
    inner: crate::multichoose::Mnr<{crate::group_count!()}, R>
}

#[cfg(feature = "multichoose")]
impl<const R: usize> GroupMnr<R> {
    pub fn new() -> Self {
        Self { inner: crate::multichoose::Mnr::new() }
    }

    /// Convert serial ID to the group choices.
    ///
    /// Returns false if the id is out of bound
    #[must_use]
    pub fn to_groups(self, id: u64, out: &mut [Group; R]) -> bool {
        let mut inner_out = [0u32; R];
        let res = self.inner.serial_to_choices(id, &mut inner_out);
        if !res {
            return false;
        }
        for i in 0..R {
            let Some(actor) = Group::from_u8(inner_out[i] as u8) else {
                return false;
            };
            // safety: serial_to_choices guarantees the value is valid
            out[i] = actor;
        }

        true
    }

    /// Convert the serial ID to the group choices, and then
    /// get the first actor in each group
    #[must_use]
    pub fn to_first_actors(self, id: u64, out: &mut [Actor; R]) -> bool {
        let mut out_groups = [Group::None; R];
        if !self.to_groups(id, &mut out_groups) {
            return false;
        }
        for i in 0..R {
            out[i] = out_groups[i].first_actor();
        }

        true
    }

    /// Convert group choices to serial ID.
    pub fn to_serial(self, groups: &[Group; R]) -> Option<u64> {
        let mut inner_actors = [0u32; R];
        for i in 0..R {
            inner_actors[i] = groups[i].as_u8() as u32;
        }
        inner_actors.sort_unstable();
        self.inner.choices_to_serial(&inner_actors)
    }

    /// Convert group choices to serial ID, by using actor choices
    pub fn to_serial_from_actors(self, actors: &[Actor; R]) -> Option<u64> {
        let mut groups = [Group::None; R];
        for i in 0..R {
            groups[i] = actors[i].group();
        }
        groups.sort_unstable();
        self.to_serial(&groups)
    }

    #[inline]
    pub fn len(self) -> u64 {
        self.inner.len()
    }
}

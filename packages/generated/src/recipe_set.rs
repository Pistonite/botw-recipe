
use bit_iter::BitIter;
/// No-alloc storage of set of recipes by their indices
/// in the static recipe data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeSet {
    /// Bit storage, from least significant to most significant,
    /// each digit is little-endian
    storage: (u64, u64, u64),
}

impl RecipeSet {
    /// Create a new recipe set
    #[inline]
    pub const fn new(a1: u64, a2: u64, a3: u64) -> Self {
        Self {
            storage: (a1, a2, a3),
        }
    }

    #[inline]
    pub const fn contains(&self, idx: usize) -> bool {
        if idx >= 138 {
            return false;
        }
        let (a1, a2, a3) = self.storage;
        let idx = idx as u64;
        if idx < 64 {
            a1 & (1 << idx) != 0
        } else if idx < 128 {
            a2 & (1 << (idx - 64)) != 0
        } else {
            a3 & (1 << (idx - 128)) != 0
        }
    }

    #[inline]
    pub const fn union(&self, other: &Self) -> Self {
        Self {
            storage: (
                self.storage.0 | other.storage.0,
                self.storage.1 | other.storage.1,
                self.storage.2 | other.storage.2,
            ),
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item=usize> {
        BitIter::from(self.storage.0)
            .chain(BitIter::from(self.storage.1).map(|x| x + 64))
            .chain(BitIter::from(self.storage.2).map(|x| x + 128))
    }
}

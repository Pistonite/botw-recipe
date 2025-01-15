
use bit_iter::BitIter;

use crate::recipe::RecipeData;

/// No-alloc storage of set of recipes by their indices
/// in the static, non-single recipe data.
///
/// There are only 13 single recipes, and they are only used
/// if there is only one unique item in the input. Therefore,
/// we don't store index data for them.
///
/// On the other hand, ther are 125 non-single recipes, which
/// fits nicely in 128 bits without the single recipes.
#[derive(Debug, Default ,Clone, PartialEq, Eq)]
pub struct RecipeSet {
    /// Bit storage, from least significant to most significant,
    /// each digit is little-endian
    storage: (u64, u64),
}

impl RecipeSet {
    /// Create a new recipe set
    #[inline]
    pub const fn new(a1: u64, a2: u64) -> Self {
        Self {
            storage: (a1, a2),
        }
    }

    #[inline]
    pub const fn contains(&self, idx: usize) -> bool {
        if idx >= crate::recipe::non_single_recipe_count() {
            return false;
        }
        let (a1, a2) = self.storage;
        let idx = idx as u64;
        if idx < 64 {
            a1 & (1 << idx) != 0
        } else {
            a2 & (1 << (idx - 64)) != 0
        }
    }

    /// Make self the union of self and other
    #[inline]
    pub const fn union(&mut self, other: &Self) {
        self.storage.0 |= other.storage.0;
        self.storage.1 |= other.storage.1;
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item=&'static RecipeData> {
        BitIter::from(self.storage.0)
            .chain(BitIter::from(self.storage.1).map(|x| x + 64))
            .map(crate::recipe::get_recipe)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_generated_recipe_set_valid() {
        for i in 0..crate::actor_count!() {
            let actor = crate::Actor::from_u8(i).unwrap();
            let recipe_set = &actor.data().matchable_recipes;
            // upper 3 bits should be clear
            assert_eq!(recipe_set.storage.1 & 0xE000_0000_0000_0000, 0);
        }
    }
}

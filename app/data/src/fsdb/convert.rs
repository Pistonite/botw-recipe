//! Conversion between recipe ID and recipe input groups
use botw_recipe_generated::Mnr;
use derive_deref::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

const NUM_GROUPS: u32 = 183;

/// A valid recipe record id
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct RecipeId(u64);
impl RecipeId {
    /// Check and create a new RecipeId. Returns None if the id is out of bounds
    pub fn new(id: u64) -> Option<Self> {
        if id < super::meta::raw_v1().total_record() {
            Some(RecipeId(id))
        } else {
            None
        }
    }

    pub fn new_unchecked(id: u64) -> Self {
        RecipeId(id)
    }
}

impl From<RecipeInputs> for RecipeId {
    fn from(items: RecipeInputs) -> Self {
        let inputs = [
            items[0].id() as u32,
            items[1].id() as u32,
            items[2].id() as u32,
            items[3].id() as u32,
            items[4].id() as u32];

        Mnr::<NUM_GROUPS, NUM_INGR>::new()
            .choices_to_serial(&inputs)
            .map(RecipeId)
            .unwrap()
    }
}

impl From<RecipeId> for u64 {
    fn from(id: RecipeId) -> Self {
        id.0
    }
}

/// A valid recipe input, which has the following invariants:
/// - Each group is a valid group
/// - The groups are sorted in ascending order
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut)]
pub struct RecipeInputs([Group; 5]);

impl From<RecipeInputs> for [Group; NUM_INGR] {
    fn from(value: RecipeInputs) -> Self {
        value.0
    }
}

impl From<[Group; NUM_INGR]> for RecipeInputs {
    fn from(mut value: [Group; NUM_INGR]) -> Self {
        value.sort_unstable();
        RecipeInputs(value)
    }
}

impl From<RecipeId> for RecipeInputs {
    fn from(id: RecipeId) -> Self {
        let mut output = [0u32; NUM_INGR];
        let res =  Mnr::<NUM_GROUPS, NUM_INGR>::new()
            .serial_to_choices(id.0, &mut output);
        debug_assert!(res);

        RecipeInputs([
            Group::from_id_unchecked(output[0] as usize),
            Group::from_id_unchecked(output[1] as usize),
            Group::from_id_unchecked(output[2] as usize),
            Group::from_id_unchecked(output[3] as usize),
            Group::from_id_unchecked(output[4] as usize),
        ])

    }
}

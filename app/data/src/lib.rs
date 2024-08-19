mod generated;

use derive_deref::Deref;
use derive_deref::DerefMut;
pub use generated::{NUM_GROUPS, NUM_INGR, NUM_TOTAL_RECORDS};
pub use generated::{Actor, Group};

/// Pre-computed numbers. MULTICHOOSE[i][m] is m multichoose i
static mut MULTICHOOSE: [[usize; NUM_GROUPS+1]; NUM_INGR+1] = [[0usize; NUM_GROUPS+1]; NUM_INGR+1];

/// Initialize the module. Please call this before anything else.
pub fn init() {
    // initialize MULTICHOOSE by using the binomial coefficient formula
    // bino[n][k] is bionmial(n, k) for k<=NUM_INGR
    let mut bino = [[0usize; NUM_INGR+1]; NUM_GROUPS+NUM_INGR];
    // base cases
    for n in 0..NUM_GROUPS+NUM_INGR {
        bino[n][0] = 1;
    }
    for k in 0..NUM_INGR+1 {
        bino[k][k] = 1;
    }
    // fill in the rest
    for n in 1..NUM_GROUPS+NUM_INGR {
        for k in 1..NUM_INGR+1 {
            bino[n][k] = bino[n-1][k-1] + bino[n-1][k];
        }
    }
    // data[i][m] is size of choosing i ingredients from m (m multichoose i)
    // MULTICHOOSE[k][n] is bino[k+n-1][k]
    for n in 0..NUM_GROUPS+1 {
        unsafe { MULTICHOOSE[0][n] = 1; }
    }
    for n in 1..NUM_INGR+1 {
        for k in 0..NUM_GROUPS+1 {
            let i = bino[n+k-1][n];
            unsafe { MULTICHOOSE[n][k] = i; }
        }
    }
}

/// Get the pre-computed number of ways to choose `k` items from `n` items, allowing for repetition
pub fn multichoose(n: usize, k: usize) -> usize {
    unsafe { MULTICHOOSE[k][n] }
}

/// A valid recipe record id
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct RecipeId(usize);
impl RecipeId {
    /// Check and create a new RecipeId. Returns None if the id is out of bounds
    pub fn new(id: usize) -> Option<Self> {
        if id < NUM_TOTAL_RECORDS {
            Some(RecipeId(id))
        }else{
            None
        }
    }
}

impl From<RecipeInputs> for RecipeId {
    fn from(items: RecipeInputs) -> Self {
        // This is the inverse of RecipeInputs::from(RecipeId)
        let mut output = 0usize;
        // reconstruct rest_items to be at the beginning of last iteration
        let mut item_lower_bound = NUM_GROUPS - items[NUM_INGR-2].id();

        // reverse the iterations
        for item in 0..NUM_INGR {
            // compute index
            let reverse_item = NUM_INGR-1-item;
            let m = items[reverse_item].id()+1;
            let mut index = 0usize;
            for reverse_m in NUM_GROUPS-item_lower_bound+1..m {
                index += multichoose(NUM_GROUPS-reverse_m+1, item);
            }
            // add to output (reverse input -= index)
            output += index;
            // recover rest_items to beginning of last iteration
            if reverse_item > 1 {
                item_lower_bound = NUM_GROUPS-items[reverse_item-2].id();
            }else{
                item_lower_bound = NUM_GROUPS;
            }
        }

        RecipeId(output)
    }
}

impl From<RecipeId> for usize {
    fn from(id: RecipeId) -> Self {
        id.0
    }
}

/// A valid recipe input, which has the following invariants:
/// - Each group is a valid group
/// - The groups are sorted in ascending order
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut)]
pub struct RecipeInputs([Group; NUM_INGR]);

impl RecipeInputs {
    /// Check and create inputs from a recipe id. Returns None if the id is out of bounds
    pub fn from_id(id: usize) -> Option<Self> {
        RecipeId::new(id).map(RecipeInputs::from)
    }
    pub fn from_groups(groups: &[Group]) -> Self {
        let len = groups.len();
        if len > NUM_INGR {
            panic!("too many inputs in recipe: {}", len);
        }
        Self::from_groups_unchecked(groups)
    }
    pub fn from_groups_unchecked(groups: &[Group]) -> Self {
        let len = groups.len();
        let mut items = [Group::None; NUM_INGR];
        for i in 0..NUM_INGR {
            if i < len {
                items[i] = groups[i];
            }else{
                items[i] = Group::None;
            }
        }
        items.into()
    }
    pub fn from_actors(actors: &[Actor]) -> Self {
        let groups = actors.iter().map(Actor::group).collect::<Vec<_>>();
        Self::from_groups(&groups)
    }
    // TODO: from groups
    // TODO: from actors
}

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
        // id is the index into the set of all recipes,
        // in the order of multichoose generation order
        // This algorithm gets the ingredients in polynomial time compared to number of materials
        let mut items = [Group::None; NUM_INGR];
        // how many ids are left
        let mut rest = id.into();
        // how many items are left (since the inputs are ascending)
        let mut item_lower_bound = NUM_GROUPS;
        
        for slot in 0..NUM_INGR {
            let mut good = false;
            // compute the slot-th item in the input array
            let mut index = 0usize;
            for m in NUM_GROUPS-item_lower_bound+1..NUM_GROUPS+1 {
                // does m overshot rest of the id
                let next_block_size = multichoose(NUM_GROUPS-m+1, NUM_INGR-1-slot);
                if index + next_block_size > rest {
                    // safety: the loop has upper bound NUM_GROUPS+1, so m-1 < NUM_GROUPS
                    items[slot] = Group::from_id_unchecked(m-1);
                    good = true;
                    break;
                }
                index += next_block_size;
            }
            if !good {
                panic!("bad recipe id: {}, when processing slot {}", usize::from(id), slot);
            }
            item_lower_bound=NUM_GROUPS-items[slot].id();
            rest -= index;
        }

        RecipeInputs(items)
    }
}


// mod multichoose;

// use std::fs;
// use bit_set::BitSet;

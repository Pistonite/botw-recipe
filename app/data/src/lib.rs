mod generated;

use std::io;
use std::io::Write;

use derive_deref::Deref;
use derive_deref::DerefMut;
pub use generated::{NUM_GROUPS, NUM_INGR, NUM_TOTAL_RECORDS, CHUNK_SIZE, CHUNK_COUNT, LAST_CHUNK_SIZE};
pub use generated::{Actor, Group};

/// Get the number of ways to choose `k` items from `n` items, allowing for repetition
///
/// The time complexity is O(1) because all values are pre-computed.
pub fn multichoose(n: usize, k: usize) -> usize {
    generated::MULTICHOOSE[n][k]
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
/// This data mirrors uking::ui::PouchItem::CookData, with an extra crit_chance field
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct CookData {
    /// Number of quarter-hearts. Usually 0-120
    pub health_recover: i32,
    /// Effect duration in seconds, usually 0-1800
    pub effect_duration: i32,
    /// Price
    pub sell_price: i32,
    /// Effect ID, but a float for some reason. -1 is None
    pub effect_id: f32,
    /// Effect level, usually 0-3, higher for hearty
    pub effect_level: f32,
    /// crit chance, usually 0-100
    pub crit_chance: i32
}

impl CookData {
    pub fn write_to<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&self.health_recover.to_le_bytes())?;
        w.write_all(&self.effect_duration.to_le_bytes())?;
        w.write_all(&self.sell_price.to_le_bytes())?;
        w.write_all(&self.effect_id.to_le_bytes())?;
        w.write_all(&self.effect_level.to_le_bytes())?;
        w.write_all(&self.crit_chance.to_le_bytes())?;
        Ok(())
    }
    /// Return an invalid CookData with all 0 bytes
    pub fn invalid() -> Self {
        Self {
            health_recover: 0,
            effect_duration: 0,
            sell_price: 0,
            effect_id: 0.0,
            effect_level: 0.0,
            crit_chance: 0
        }
    }

    pub fn is_invalid(&self) -> bool {
        return self == &Self::invalid();
    }

    /// Get that the data are in their normal ranges
    pub fn is_normal(&self) -> bool {
        if self.health_recover < 0 || self.health_recover > 120 {
            return false;
        }
        if self.effect_duration < 0 || self.effect_duration > 1800 {
            return false;
        }
        if self.sell_price < 0 {
            return false;
        }
        if self.effect_level < 0.0 {
            return false;
        }
        if self.effect_level.round() != self.effect_level {
            return false;
        }
        match self.effect_id {
            MOD_NONE => {
                if self.effect_level != 0.0 || self.effect_duration != 0 {
                    return false;
                }
            }
            MOD_LIFE_RECOVER => {} //what is this??
            MOD_LIFE_MAX_UP => {
                if self.effect_duration != 0 {
                    return false;
                }
                if self.effect_level > 25.0 {
                    // max is 5 big hearty radish which gives 25 hearts
                    return false;
                }
            }
            MOD_RESIST_HOT | MOD_RESIST_COLD | MOD_FIREPROOF=> {
                if self.effect_level> 2.0 {
                    return false;
                }
            }
            MOD_RESIST_ELECTRIC |
MOD_ATTACK_UP | MOD_DEFENSE_UP | MOD_QUIETNESS | MOD_MOVING_SPEED
=> {
                if self.effect_level > 3.0 {
                    return false;
                }
            }
            MOD_GUTS_RECOVER => {} //TODO
            MOD_EX_GUTS_MAX_UP => { } //TODO
            _ => {
                return false;
            }
        }

        true
    }
}
const MOD_LIFE_RECOVER :f32= 1.0;
const MOD_LIFE_MAX_UP :f32= 2.0;
const MOD_RESIST_HOT :f32= 4.0;
const MOD_RESIST_COLD :f32= 5.0;
const MOD_RESIST_ELECTRIC :f32= 6.0;
const MOD_ATTACK_UP: f32 = 10.0;
const MOD_DEFENSE_UP :f32= 11.0;
const MOD_QUIETNESS :f32= 12.0;
const MOD_MOVING_SPEED :f32= 13.0;
const MOD_GUTS_RECOVER :f32= 14.0;
const MOD_EX_GUTS_MAX_UP :f32= 15.0;
const MOD_FIREPROOF :f32= 16.0;
const MOD_NONE :f32= -1.0;
